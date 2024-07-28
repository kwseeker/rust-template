/// OpenAI 客户端
#[derive(Clone)]
pub(crate) struct OpenAI {
    // RustGLM 中存储的只是每次输入和输出，并不是不变的配置，不适合放在这里
    // client: RustGLM::RustGLM,
    api_key: String,
    glm_version: &'static str,
    config_file: &'static str,
}

impl OpenAI {
    pub(crate) fn new() -> Self {
        let api_key = std::env::var("CHATGLM_API_KEY").unwrap();
        println!("CHATGLM_API_KEY: {api_key}");
        OpenAI {
            api_key,
            glm_version: "glm-4",
            config_file: "Constants.toml",
        }
    }

    pub(crate) async fn code_review(&self, code_diff: String) -> String {
        println!("Review Begin ...");
        let mut rust_glm = RustGLM::RustGLM::new().await;
        rust_glm.set_user_input(code_diff);
        let ai_response = rust_glm
            .rust_chat_glm(Some(self.api_key.clone()), self.glm_version.clone(),self.config_file.clone()).await;
        println!("Review Response: {ai_response}");
        ai_response
    }
}

#[cfg(test)]
mod tests {
}