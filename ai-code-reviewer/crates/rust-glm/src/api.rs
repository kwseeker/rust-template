/// API invocation
/// Api URL: https://open.bigmodel.cn/api/paas/v4/chat/completions, docs: https://open.bigmodel.cn/dev/api#glm-4

use anyhow::bail;
use futures_util::StreamExt;
use log::debug;
use crate::context;
use crate::context::ContextMessage;
use crate::message::{AssistantMessage, Message, MessageType, Query, UserMessage};
use crate::models::glm4;
use crate::models::glm4::{ResponseChunk, Usage};

lazy_static::lazy_static! {
    static ref UNICODE_REGEX: regex::Regex = regex::Regex::new(r"\\u[0-9a-fA-F]{4}").unwrap();
}

const RESPONSE_DONE: &str = "data: [DONE]";

// SSE
pub(crate) async fn invoke_sse(query: Query<'_>) -> anyhow::Result<String> {
    let jwt = query.jwt();
    let query_message = query.message().clone();
    let request_body = glm4::RequestBody::assemble(query);
    let request = reqwest::Client::new()
        .post(glm4::API_URL)
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .header("Accept", "text/event-stream")
        .header("Content-Type", "application/json;charset=UTF-8")
        .header("Authorization", format!("Bearer {}", jwt))
        .json(&request_body);
    let response = request.send().await?;
    // exception handling
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Failed to read error text".into());
        bail!("Request failed with status: {}, error: {}", status, error_text);
    }
    // parse response
    let mut response_body = response.bytes_stream();
    let mut sse_chunks = String::new();
    while let Some(chunk) = response_body.next().await {
        match chunk {
            Ok(bytes) => {
                let data = String::from_utf8_lossy(&bytes);
                // finally statistics data and "data: [DONE]" maybe send back in one response body
                if data.contains(RESPONSE_DONE) {
                    let ri = data.rfind(RESPONSE_DONE).unwrap();
                    let front_content = &data[..ri];
                    if front_content.contains("data: ") {
                        let li = data.find("data: ").unwrap();
                        let statistics_content = &front_content[li..];
                        sse_chunks.push_str(statistics_content);
                    }
                    break;
                }
                let data = data.trim_start_matches("data: ");
                // debug!("chunk: {}", data.to_string());
                sse_chunks.push_str(data);
                let response_chunk = ResponseChunk::from_string(data);
                response_chunk.print();
            }
            Err(e) => {
                bail!("Error receiving SSE event: {}", e);
            }
        }
    }
    debug!("SSE chunks: {}", sse_chunks);

    invoke_sse_post_process(query_message, sse_chunks.clone())?;
    Ok(sse_chunks)
}

pub(crate) fn invoke_sse_post_process(query_message: String, response_chunks: String) -> anyhow::Result<()> {
    let chunks: Vec<&str> = response_chunks.lines()
        .map(|line| line.trim_start_matches("data: "))
        .filter(|line| !line.is_empty())
        .collect();

    let mut complete_content = String::new();
    let mut usage= None;
    for chunk in chunks {
        let chunk = ResponseChunk::from_string(chunk);
        let chunk_content = chunk.choices()[0].delta().content();
        let content = convert_unicode_emojis(chunk_content)
            .replace("\"", "")
            .replace("\\n\\n", "\n")
            .replace("\\nn", "\n")
            .replace("\\\\n", "\n")
            .replace("\\\\nn", "\n")
            .replace("\\", "");
        complete_content.push_str(&content);

        //count tokens cost, last chunk will return prompt and completion token costs
        if let Some(u) = chunk.usage() {
            usage = Some(u.clone());
        }
    }

    if !complete_content.is_empty() {
        // store to context file
        let user_message = UserMessage::new(query_message);
        let assistance_message = AssistantMessage::new(complete_content);
        let (prompt_tokens, completion_tokens) = match usage {
            Some(usage) => (usage.prompt_tokens(), usage.completion_tokens()),
            None => (0, 0)
        };
        let context_messages = vec![
            ContextMessage::new(MessageType::User(user_message), prompt_tokens),
            ContextMessage::new(MessageType::Assistant(assistance_message), completion_tokens)];
        let writer = context::Writer::new();
        writer.append(context_messages)?;
    }

    Ok(())
}

fn convert_unicode_emojis(input: &str) -> String {
    UNICODE_REGEX.replace_all(input, |caps: &regex::Captures| {
        let emoji = char::from_u32(
            u32::from_str_radix(&caps[0][2..], 16).expect("Failed to parse Unicode escape")
        )
            .expect("Invalid Unicode escape");
        emoji.to_string()
    })
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::api::RESPONSE_DONE;

    #[test]
    fn response_data() {
        let str = r#"data: {"id":"20240806145645934a33f99eae4dc9","created":1722927405,"model":"glm-4","choices":[{"index":0,"finish_reason":"stop","delta":{"role":"assistant","content":""}}],"usage":{"prompt_tokens":56,"completion_tokens":8,"total_tokens":64}}
data: [DONE]
"#;
        let idx = str.rfind(RESPONSE_DONE).unwrap();
        let prefix = &str[..idx];
        println!("{prefix}")
    }
}