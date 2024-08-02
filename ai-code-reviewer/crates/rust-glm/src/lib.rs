mod custom_jwt;
mod api_operation;
mod async_invoke_method;
mod sync_invoke_method;
mod sse_invoke_method;
mod glm_client;
mod message;
mod init;
mod api;
mod jwt;
mod models;

use std::collections::HashMap;
use std::sync::Arc;
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use regex::Regex;

/// 写法有点奇怪，输入输出并不是本体，不像OOP的写法， TODO 重构
#[derive(Debug)]
pub struct RustGLM {
    chatglm_response: String,
    chatglm_input: String,
}

enum CallResult {
    Success(String),
    Error(String),
}

impl RustGLM {
    pub async fn new() -> Self {
        RustGLM {
            chatglm_response: String::new(),
            chatglm_input: String::new(),
        }
    }

    pub fn set_user_input(&mut self, input: String) {
        self.chatglm_input = input;
    }

    async fn async_invoke_calling(jwt_token: &str, user_input: &str, glm_version: &str, user_config: &str) -> String {
        let jwt_token_clone = jwt_token.to_string();
        let user_input_clone = user_input.to_string();
        let glm_version_clone = glm_version.to_string();
        let user_config_clone = user_config.to_string();

        let handle = tokio::spawn(async move {
            let response =
                async_invoke_method::ReceiveAsyncInvokeOnlyText::new(&jwt_token_clone, &user_input_clone, &glm_version_clone, user_config_clone);
            response
                .await
                .get_response()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Error getting response.".to_string())
        });

        handle.await.expect("Failed to await JoinHandle")
    }

    async fn sync_invoke_calling(jwt_token: &str, user_input: &str, glm_version: &str, user_config: &str) -> String {
        let sync_call = sync_invoke_method::ReceiveInvokeModelOnlyText::new(jwt_token, user_input, glm_version, user_config);

        match sync_call.await.get_response_message() {
            Some(message) => message.to_string(), // Return the message as String
            None => "Error: Unable to get sync response.".to_string(),
        }
    }

    async fn sse_invoke_calling(jwt_token: &str, user_input: &str, glm_version: &str, user_config: &str) -> String {
        let sse_call = sse_invoke_method::ReceiveSSEInvokeModelOnlyText::new(jwt_token, user_input, glm_version, user_config);

        match sse_call.await.get_response_message() {
            Some(message) => message.to_string(), // Return the message as String
            None => "Error: Unable to get SSE response.".to_string(),
        }
    }


    async fn call_sse(jwt: Arc<String>, user_in: &str, user_glm_version: &str, user_config: &str) -> String {
        Self::sse_invoke_calling(&jwt, user_in, user_glm_version, user_config).await
    }


    async fn call_sync(jwt: Arc<String>, user_in: &str, user_glm_version: &str, user_config: &str) -> String {
        Self::sync_invoke_calling(&jwt, user_in, user_glm_version, user_config).await
    }

    async fn call_async(jwt: Arc<String>, user_in: &str, user_glm_version: &str, user_config: &str) -> String {
        Self::async_invoke_calling(&jwt, user_in, user_glm_version, user_config).await
    }

    async fn regex_checker(regex: &Regex, input: &str) -> bool {
        regex.is_match(&*input)
    }

    /// 这个其实是发起请求，不是名字上的验证调用是否有效
    async fn is_call_valid(
        part1_content: String,
        part2_content: Arc<String>,
        glm_version: Arc<String>,
        user_config: Arc<String>,
        jwt: Arc<String>,
    ) -> CallResult {
        let mut methods: HashMap<&str, Box<dyn Fn() -> BoxFuture<'static, String> + Send>> =
            HashMap::new();
        let jwt_for_sse = Arc::clone(&jwt);
        let jwt_for_async = Arc::clone(&jwt);
        let jwt_for_sync = Arc::clone(&jwt);

        let user_config_sse = Arc::clone(&user_config);
        let user_config_async = Arc::clone(&user_config);
        let user_config_sync = Arc::clone(&user_config);

        let glm_version_sse = Arc::clone(&glm_version);
        let glm_version_async = Arc::clone(&glm_version);
        let glm_version_sync = Arc::clone(&glm_version);

        let part2_content_sse = Arc::clone(&part2_content);
        let part2_content_async = Arc::clone(&part2_content);
        let part2_content_sync = Arc::clone(&part2_content);

        methods.insert("sse", Box::new(move || {
            let jwt_for_sse = Arc::clone(&jwt_for_sse);
            let part2_content = Arc::clone(&part2_content_sse);
            let user_glm_version = Arc::clone(&glm_version_sse);
            let user_config = Arc::clone(&user_config_sse);
            async move {
                RustGLM::call_sse(jwt_for_sse, part2_content.trim(), &user_glm_version, &user_config).await
            }
                .boxed()
        }));

        methods.insert("async", Box::new(move || {
            let jwt_for_async = Arc::clone(&jwt_for_async);
            let part2_content = Arc::clone(&part2_content_async);
            let user_glm_version = Arc::clone(&glm_version_async);
            let user_config = Arc::clone(&user_config_async);
            async move {
                RustGLM::call_async(jwt_for_async, part2_content.trim(), &user_glm_version, &user_config).await
            }
                .boxed()
        }));

        methods.insert("sync", Box::new(move || {
            let jwt_for_sync = Arc::clone(&jwt_for_sync);
            let part2_content = Arc::clone(&part2_content_sync);
            let user_glm_version = Arc::clone(&glm_version_sync);

            let user_config = Arc::clone(&user_config_sync);
            async move {
                RustGLM::call_sync(jwt_for_sync, part2_content.trim(), &user_glm_version, &user_config).await
            }
                .boxed()
        }));

        loop {
            match part1_content.trim().to_lowercase().as_str() {
                "exit" => break,
                method => {
                    return if let Some(call_invoke) = methods.get(method) {
                        //let require_calling = method.to_string().to_uppercase();
                        //println!("Calling method is {}", require_calling);
                        let future = call_invoke();
                        let ai_message = future.await;
                        CallResult::Success(ai_message)
                    } else {
                        CallResult::Error("Invalid method".to_string())
                    }
                }
            }
        }
        CallResult::Error("Unknown error".to_string())
    }

    /// 异步调用 Rust Chat GLM 接口，用于处理自然语言处理任务。
    ///
    /// RustGLM 调用 ChatGLM API 交互流程
    /// 1.传参的方式传递请求配置
    /// 2.解析 API_KEY, 并使用解析结果创建 JWT 令牌并自校验
    /// 3.为每种调用方式设置一个闭包，用于向 ChatGLM 发起请求
    /// # Arguments
    /// * `api_key` - ChatGLM API KEY
    /// * `glm_version` - 要使用的 ChatGLM 版本
    /// * `user_config` - 用户配置的 JSON 字符串，用于自定义请求行为。
    /// # Returns
    /// * 返回 ChatGLM 接口响应结果
    pub async fn rust_chat_glm(&mut self, api_key: Option<String>, glm_version: &str, user_config: &str) -> String {
        let user_in = &self.chatglm_input;
        let (mut part1_content, mut part2_content) = ("SSE".to_string(), String::new());

        // 命令行每次可以选择使用调用方式 sse(默认) async sync，还有个 exit 用于退出， 比如 sse#<用户输入消息>
        let regex_input = Regex::new(r"([^#]+)#([^#]+)").unwrap();
        if let Some(captures_message) = regex_input.captures(user_in) {
            part1_content = captures_message.get(1).map_or_else(|| "SSE".to_string(), |m| m.as_str().to_string());
            // part2 就是用户输入消息
            part2_content = captures_message.get(2).map_or_else(|| String::new(), |m| m.as_str().to_string());
        } else if !Self::regex_checker(&regex_input, &*user_in.clone()).await {
            part2_content = user_in.trim().to_string();
        } else {
            CallResult::Error("Input does not match the pattern".to_string());
            return String::new();
        }

        if let Some(api_key) = api_key {
            let api_key_instance = api_operation::APIKeys::get_instance(&api_key);
            let jwt_creator = custom_jwt::CustomJwt::new(api_key_instance.get_user_id(), api_key_instance.get_user_secret());
            let jwt = Arc::new(jwt_creator.create_jwt());

            if !jwt_creator.verify_jwt(&jwt) {
                CallResult::Error("Error Code: 1200, API Key not found or an error occurred while loading.".to_string());
                return String::new();
            }

            if let CallResult::Success(ai_message) = Self::is_call_valid(
                part1_content,
                Arc::new(part2_content),
                Arc::new(glm_version.to_string()),
                Arc::new(user_config.to_string()),
                jwt,
            ).await {
                return ai_message;
            }
        } else {
            CallResult::Error("Error Code: 1200, API Key not found or an error occurred while loading.".to_string());
        }

        String::new()
    }

    pub fn get_ai_response(&self) -> String {
        self.chatglm_response.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::RustGLM;

    #[tokio::test]
    async fn rust_chat_glm() {
        let mut rust_glm = RustGLM::new().await;
        let api_key = std::env::var("API_KEY").unwrap();
        let user_in = String::from("SSE#讲个笑话");
        rust_glm.set_user_input(user_in);
        // Constants.toml 中是调用环境设置、以及聊天预设
        let ai_response =
            rust_glm.rust_chat_glm(Some(api_key), "glm-4", "Constants.toml").await;
        assert!(!ai_response.is_empty());
    }
}