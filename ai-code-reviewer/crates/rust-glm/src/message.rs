use regex::Regex;
use crate::{glm_client, jwt};

pub(crate) trait Message {}

/// ChatGLM provide 4 types of Message
pub(crate) enum MessageType {
    /// preset chat scene
    System,
    /// user query message
    User,
    /// AI model reply message
    Assistant,
    /// message generated by tools
    Tool,
}

pub(crate) struct SystemMessage {
    role: String,
    content: String,
}

impl Message for SystemMessage {}

pub(crate) struct UserMessage {
    role: String,
    content: String,
}

impl Message for UserMessage {}

pub(crate) struct AssistantMessage {
    role: String,
    content: Option<String>,
    tool_calls: Option<Vec<ToolCall>>,
}

impl Message for AssistantMessage {}

struct ToolCall {
    id: String,
    _type: String,
    function: Option<String>,
}

pub(crate) struct ToolMessage {
    role: String,
    content: String,
    tool_call_id: String,
}

impl Message for ToolMessage {}

/// user query message
pub struct Query<'c> {
    config: &'c glm_client::Config,
    /// transfer mode: Sse Sync Async
    trans_mode: TransferMode,
    /// message this query
    message: String,
    // GLM model
    // glm_model: &'m GlmModel,
    /// call ChatGLM API need jwt token
    jwt: String,
}

impl Query {
    /// construct query from chat message and client configuration
    pub fn from_string<'c>(input: &str, config: &'c glm_client::Config) -> Query {
        let (mut trans_type, mut message) = (String::new(), String::new());

        // every query can choose different invoke methods: sse(default) async sync，for example sse#<user_input>
        let regex = Regex::new(r"([^#]+)#([^#]+)").unwrap();
        // regex 正则匹配 query 字符串, 如果匹配提取各个部分
        if let Some(captures_message) = regex.captures(input) {
            trans_type = captures_message.get(1).map_or_else(
                || String::new(), |m| m.as_str().to_string());
            message = captures_message.get(2).map_or_else(
                || String::new(), |m| m.as_str().to_string());
        } else {
            message = input.trim().to_string();
        }

        Query {
            config,
            trans_mode: TransferMode::from_string(&trans_type),
            message,
            // glm_model: config.glm_model(),
            // get or create jwt token
            // if not exist or about to expire create new one, otherwise reuse
            jwt: jwt::JWT_HOLDER.get_jwt(),
        }
    }

    pub(crate) fn config(&self) -> &glm_client::Config {
        self.config
    }

    pub(crate) fn trans_mode(&self) -> &TransferMode {
        &self.trans_mode
    }

    pub(crate) fn jwt(&self) -> String {
        self.jwt.clone()
    }
}

/// ChatGLM reply message
struct Reply {}

pub(crate) enum TransferMode {
    /// stream
    Sse,
    Sync,
    Async,
}

impl TransferMode {
    fn from_string(mode: &str) -> TransferMode {
        match mode {
            "sse" => TransferMode::Sse,
            "sync" => TransferMode::Sync,
            "async" => TransferMode::Async,
            _ => TransferMode::Sse,
        }
    }
}