/// API invocation
/// Api URL: https://open.bigmodel.cn/api/paas/v4/chat/completions, docs: https://open.bigmodel.cn/dev/api#glm-4

use anyhow::bail;
use futures_util::StreamExt;
use log::debug;
use crate::message::Query;
use crate::models::glm4;

// SSE
pub(crate) async fn invoke_sse(query: Query) -> anyhow::Result<String> {
    let request_body = glm4::RequestBody::assemble(query);
    let request = reqwest::Client::new()
        .post(glm4::API_URL)
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .header("Accept", "text/event-stream")
        .header("Content-Type", "application/json;charset=UTF-8")
        .header("Authorization", format!("Bearer {}", query.jwt()))
        .json(request_body);
    let response = request.send().await?;
    // exception handling
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Failed to read error text".into());
        return bail!("Request failed with status: {}, error: {}", response.status(), error_text);
    }
    // parse response
    let mut response_body = response.bytes_stream();
    let mut sse_data = String::new();
    while let Some(chunk) = response_body.next().await {
        match chunk {
            Ok(bytes) => {
                let data = String::from_utf8_lossy(&bytes);
                sse_data.push_str(&data);
                if data.contains("data: [DONE]") {  //TODO check
                    break;
                }
            }
            Err(e) => {
                return bail!("Error receiving SSE event: {}", e);
            }
        }
    }
    debug!("SSE data: {}", sse_data);
    Ok(sse_data)
}


// trait Invoke {
//     fn invoke(&self, command: &str) -> CallResult;
// }
//
// struct Invoker {
//     map: HashMap<String, Box<dyn Fn(&str) -> CallResult>>,
// }
//
// impl Invoker {
//     pub fn new() -> Invoker {
//         Invoker {
//         }
//     }
// }
//
// struct SseInvoker {
//
// }

