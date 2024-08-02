use serde_derive::{Deserialize, Serialize};
use crate::message::{Message, Query};

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
    messages: Vec<dyn Message>,
    do_sample: Option<bool>,
    stream: bool,
    temperature: Option<f32>,
    top_p: Option<f32>,
    max_tokens: Option<u32>,
    stop: Option<Vec<String>>,
    user_id: Option<String>,
}

impl RequestBody {
    pub(crate) fn assemble(query: Query) -> &RequestBody {
        let mut request_body = RequestBody::default();

        let config = query.config();
        request_body.model = config.glm_model().to_string();
        request_body.messages = vec![]; //TODO
        request_body.user_id = None;    //TODO
        request_body.stream = true;     //TODO

        let glm4_config = config.glm4_config();
        if let Some(glm4_config) = glm4_config {
            request_body.do_sample = glm4_config.do_sample;
            request_body.temperature = glm4_config.temperature;
            request_body.top_p = glm4_config.top_p;
            request_body.max_tokens = glm4_config.max_tokens;
            request_body.stop = glm4_config.stop.clone();
        }
        &request_body
    }
}

pub(crate) struct ResponseBody {

}