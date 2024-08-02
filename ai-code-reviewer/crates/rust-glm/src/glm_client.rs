use std::fs::File;
use std::io::Read;
use log::error;
use serde_derive::Deserialize;
use crate::{api, init};
use crate::message::{Query, TransferMode};
use crate::models::glm4::Glm4Config;
use crate::models::GlmModel;

/// GLM client configuration
#[derive(Clone)]
pub struct Config {
    api_key: Option<ApiKey>,
    /// GLM language model
    glm_model: GlmModel,
    glm_4: Option<Glm4Config>,
}

const CONFIG_FILE: &str = "client.toml";

impl Default for Config {
    fn default() -> Config {
        let mut config = Config {
            api_key: ApiKey::load_from_env(),
            glm_model: GlmModel::Glm4,
            glm_4: None,
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
                config.glm_4 = llc.glm_4;
            }
            Err(err) => error!("Error to load config from toml: {}", err)
        }
        config
    }
}

impl Config {
    pub fn glm_model(&self) -> &GlmModel {
        &self.glm_model
    }

    pub fn glm4_config(&self) -> &Option<Glm4Config> {
        &self.glm_4
    }
}

#[derive(Deserialize)]
struct LowLevelConfig {
    api_key: Option<String>,
    glm_model: Option<String>,
    log_level: Option<String>,
    glm_4: Option<Glm4Config>,
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

    // support modified by params

    pub fn glm_model(&mut self, glm_model: GlmModel) -> &mut GlmClientBuilder {
        self.config.glm_model = glm_model;
        self
    }

    pub fn build(&self) -> GlmClient {
        GlmClient {
            config: self.config.clone(),
        }
    }
}

/// GLM Rust SDK main struct
struct GlmClient {
    config: Config,
    // invoker: Invoker,
}

impl GlmClient {
    /// GlmClient call ChatGLM API（ https://open.bigmodel.cn/api/paas/v4/chat/completions）process
    /// 1.
    pub fn chat(&self, message: &str) -> () {
        let query = Query::from_string(message, &self.config);
        match query.trans_mode() {
            TransferMode::Sse => {
                api::invoke_sse(query);
            }
            TransferMode::Sync => {
                // sync
            }
            TransferMode::Async => {
                // async
            }
        }
    }


}

#[derive(Clone)]
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

#[cfg(test)]
mod tests {
    use crate::glm_client::{GlmClientBuilder, LowLevelConfig};

    #[tokio::test]
    async fn chat() {
        ///
        let glm_client = GlmClientBuilder::new().build();
        glm_client.chat("1+1=?");

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