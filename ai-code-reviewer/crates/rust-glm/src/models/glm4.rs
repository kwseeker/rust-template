use log::debug;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use crate::message::{Message, MessageType, Query, SystemMessage, UserMessage};
use crate::printer;

pub(crate) static API_URL: &str = "https://open.bigmodel.cn/api/paas/v4/chat/completions";

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Glm4Config {
    model: String,
    do_sample: Option<bool>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    max_tokens: Option<u32>,
    stop: Option<Vec<String>>,
    system_content: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct RequestBody {
    model: String,
    messages: Vec<Value>,
    do_sample: Option<bool>,
    stream: bool,
    temperature: Option<f32>,
    top_p: Option<f32>,
    max_tokens: Option<u32>,
    stop: Option<Vec<String>>,
    user_id: Option<String>,
}

impl RequestBody {
    pub(crate) fn assemble(query: Query) -> RequestBody {
        let mut request_body = RequestBody::default();

        let config = query.config();
        request_body.model = config.glm_model().to_string();
        request_body.messages = Self::assemble_messages(config.glm4_config(), query.message());
        request_body.user_id = Some(config.api_key().user_id().to_string());
        request_body.stream = query.trans_mode().is_stream();

        let glm4_config = config.glm4_config();
        if let Some(glm4_config) = glm4_config {
            request_body.do_sample = glm4_config.do_sample;
            request_body.temperature = glm4_config.temperature;
            request_body.top_p = glm4_config.top_p;
            request_body.max_tokens = glm4_config.max_tokens;
            request_body.stop = glm4_config.stop.clone();
        }
        debug!("Request body: {:#?}", serde_json::to_string(&request_body).unwrap());
        request_body
    }

    /// Assemble system message、user and assistance history message、current user message etc
    fn assemble_messages(glm4_config: &Option<Glm4Config>, current_message: &String) -> Vec<Value> {
        let mut messages: Vec<Value> = Vec::new();
        // System messages
        if let Some(config) = glm4_config {
            if let Some(content) = config.clone().system_content {
                messages.push(SystemMessage::new(content).to_value());
            }
        }
        // History messages

        // Current user messages
        messages.push(UserMessage::new(current_message.clone()).to_value());
        // Tool Messages, temporarily ignore

        messages
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ResponseChunk {
    id: String,
    created: u64,
    choices: Vec<Choice>,
    usage: Option<Usage>,
    web_search: Option<Vec<WebSearch>>,
}

impl ResponseChunk {
    pub(crate) fn from_string(chunk_json: &str) -> ResponseChunk {
        // let chunk_json = reply.trim_start_matches("data: ");
        let block: ResponseChunk = serde_json::from_str(chunk_json).unwrap();
        block
    }

    pub(crate) fn print(&self) {
        for choice in &self.choices {
            printer::Standard::print(choice.delta.content.clone());
        }
    }

    pub(crate) fn choices(&self) -> &Vec<Choice> {
        &self.choices
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Choice {
    index: u32,
    finish_reason: Option<String>,
    delta: Delta,
}

impl Choice {
    pub(crate) fn delta(&self) -> &Delta {
        &self.delta
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Delta {
    role: String,
    content: String,
    // tool_calls: Vec<ToolCall>
}

impl Delta {
    pub(crate) fn content(&self) -> &String {
        &self.content
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct WebSearch {
    icon: String,
    title: String,
    link: String,
    media: String,
    content: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn response_chunk() {
        let str = r#"{"id":"20240803193154672357f4d3ac4e2b","created":1722684714,"model":"glm-4","choices":[{"index":0,"delta":{"role":"assistant","content":"1"}}]}"#;
        let response_chunk = super::ResponseChunk::from_string(str);
        response_chunk.print();
    }
}