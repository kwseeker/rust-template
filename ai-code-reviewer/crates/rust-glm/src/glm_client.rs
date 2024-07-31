use std::fs::File;
use std::io::Read;
use anyhow::anyhow;
use log::error;
use serde_derive::Deserialize;
use crate::init;

/// GLM client configuration
struct Config {
    api_key: Option<ApiKey>,
    /// GLM language model
    glm_model: GlmModel,
}

const CONFIG_FILE: &str = "client.toml";

impl Default for Config {
    fn default() -> Config {
        let mut config = Config {
            api_key: ApiKey::load_from_env(),
            glm_model: GlmModel::Glm4,
        };
        // load settings from toml file if exists and override
        let low_level_config = LowLevelConfig::load_from_toml();
        match low_level_config {
            Ok(llc) => {
                if let Some(api_key) = llc.api_key {
                    config.api_key = ApiKey::from_string(api_key);
                }
                if let Some(glm_model) = llc.glm_model {
                    config.glm_model = GlmModel::from_string(glm_model);
                }
            }
            Err(err) => error!("Error to load config from toml: {}", err)
        }
        config
    }
}

#[derive(Deserialize)]
struct LowLevelConfig {
    api_key: Option<String>,
    glm_model: Option<String>,
    log_level: Option<String>,
}

impl LowLevelConfig {
    fn load_from_toml() -> anyhow::Result<LowLevelConfig> {
        let mut file = File::open(CONFIG_FILE)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: LowLevelConfig = toml::from_str(&contents)?;
        config.init_log();
        Ok(config)
    }

    fn init_log(&self) {
        init::init_log(self.log_level.clone())
    }
}

struct GlmClientBuilder {
    config: Config,
}

impl GlmClientBuilder {
    pub fn new() -> GlmClientBuilder {
        GlmClientBuilder {
            config: Config::default(),
        }
    }

    // 支持传参修改配置

    pub fn glm_model(&mut self, glm_model: GlmModel) -> &mut GlmClientBuilder {
        self.config.glm_model = glm_model;
        self
    }

    pub fn build(&self) -> GlmClient {
        GlmClient {

        }
    }
}

/// GLM Rust SDK main struct
struct GlmClient {

}

impl GlmClient {
    pub fn new() -> Self {
        GlmClient {}
    }

    /// GlmClient 调用 ChatGLM API 流程
    /// 2.解析 API_KEY, 并使用解析结果创建 JWT 令牌并自校验
    /// 3.为每种调用方式设置一个闭包，用于向 ChatGLM 发起请求
    pub fn chat(&self, message: String) -> () {}
}

struct ApiKey {
    user_id: String,
    secret_key: String,
}

impl ApiKey {
    fn load_from_env() -> Option<ApiKey> {
        //从环境变量中获取
        let api_key = std::env::var("CHATGLM_API_KEY");
        match api_key {
            Ok(api_key) => {
                let api_key_parts: Vec<&str> = api_key.split('.').collect();
                if api_key_parts.len() != 2 {
                    return None;
                }
                let user_id = api_key_parts[0].to_string();
                let secret_key = api_key_parts[1].to_string();
                Some(ApiKey {
                    user_id,
                    secret_key,
                })
            }
            Err(err) => {
                error!("Error to load api key from env: {}", err);
                None
            }
        }
    }

    fn from_string(api_key: String) -> Option<ApiKey> {
        let api_key_parts: Vec<&str> = api_key.split('.').collect();
        if api_key_parts.len() != 2 {
            return None;
        }
        let user_id = api_key_parts[0].to_string();
        let secret_key = api_key_parts[1].to_string();
        Some(ApiKey {
            user_id,
            secret_key,
        })
    }
}

/// SDK暂时只支持两种模型
enum GlmModel {
    Glm4,
    Glm4_0520,
}

impl GlmModel {
    fn from_string(model: String) -> GlmModel {
        match model.as_str() {
            "glm-4" => GlmModel::Glm4,
            "glm-4-0520" => GlmModel::Glm4_0520,
            _ => panic!("Invalid model"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::glm_client::{Config, GlmClient, GlmClientBuilder, LowLevelConfig};

    #[tokio::test]
    async fn chat() {
        ///
        let glm_client = GlmClientBuilder::new().build();
        glm_client.chat("".to_string());

        // // Constants.toml 中是调用环境设置、以及聊天预设
        // let ai_response =
        //     rust_glm.rust_chat_glm(Some(api_key), "glm-4", "Constants.toml").await;
        // assert!(!ai_response.is_empty());
    }

    #[test]
    fn load_from_toml() {
        let config = LowLevelConfig::load_from_toml();
        assert!(config.is_ok())
    }
}