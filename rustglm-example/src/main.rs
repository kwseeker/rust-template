use std::io;

/// 注意如果免费体验包过期的话，就只能付费调用智谱的模型API了，
/// 接口响应 Server returned an error: 429 Too Many Requests 其实就是欠费没有请求额度了
#[tokio::main]
async fn main() {
    let mut rust_glm = RustGLM::RustGLM::new().await;
    loop {
        // 从环境变量加载 API Key
        let api_key = std::env::var("API_KEY").unwrap();

        println!("You:");
        let mut user_in = String::new();
        io::stdin().read_line(&mut user_in).expect("Failed to read line");
        rust_glm.set_user_input(user_in.trim().to_string()); // 使用修改后的 RustGLM 实例

        // Constants.toml 中是调用环境设置、以及聊天预设
        let ai_response = rust_glm.rust_chat_glm(Some(api_key),"glm-4","Constants.toml").await;
        println!("Liliya: {}", ai_response);

        if ai_response.is_empty() {
            break;
        }
        println!();
    }
}