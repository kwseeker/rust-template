use std::sync::{Arc, OnceLock};
use tokio::runtime::Runtime;

/// OpenAI 客户端
pub(crate) struct OpenAI {
    client: RustGLM::RustGLM,
    api_key: String,
}

impl OpenAI {
    pub(crate) fn new(runtime: Arc<Runtime>) -> &'static Self {
        // 避免重复创建
        static P: OnceLock<OpenAI> = OnceLock::new();
        P.get_or_init(|| {                              //OnceLock 控制的变量只会写入一次（且是同步写入）
            let api_key = std::env::var("CHATGLM_API_KEY").unwrap();
            println!("CHATGLM_API_KEY: {api_key}");
            OpenAI {
                client: runtime.block_on(RustGLM::RustGLM::new()),
                api_key,
            }
        })
    }

    pub(crate) fn code_review(&self, code_diff: String) {
        println!("code_review: ...");
        // self.client.set_user_input(code_diff);

        //     rust_glm.set_user_input(user_in.trim().to_string()); // 使用修改后的 RustGLM 实例
        //
        //     // Constants.toml 中是调用环境设置、以及聊天预设
        //     let ai_response = rust_glm.rust_chat_glm(Some(api_key),"glm-4","Constants.toml").await;
        //     println!("Liliya: {}", ai_response);
        //
        //     if ai_response.is_empty() {
        //         break;
        //     }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn connect_openai() {}
}