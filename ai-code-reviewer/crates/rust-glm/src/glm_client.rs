use std::fs::File;
use std::io::Read;
use log::{debug, error};
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

    pub fn api_key(&self) -> &ApiKey {
        match &self.api_key {
            Some(api_key) => {
                return api_key;
            }
            None => panic!("Error: api key not existed!")
        }
    }
}

#[derive(Deserialize, Debug)]
struct LowLevelConfig {
    api_key: Option<String>,
    glm_model: Option<String>,
    log_level: Option<String>,
    glm_4: Option<Glm4Config>,
}

impl LowLevelConfig {
    fn load_from_toml() -> anyhow::Result<LowLevelConfig> {
        let mut file = File::open(CONFIG_FILE)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Self::from_string(content)
    }

    fn from_string(content: String) -> anyhow::Result<LowLevelConfig> {
        let config: LowLevelConfig = toml::from_str(&content)?;
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
    pub async fn chat(&self, message: &str) -> anyhow::Result<()> {
        let query = Query::from_string(message, &self.config);
        match query.trans_mode() {
            TransferMode::Sse => {
                api::invoke_sse(query).await?;
            }
            TransferMode::Sync => {
                // sync
            }
            TransferMode::Async => {
                // async
            }
        }

        debug!("chat done!");
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct ApiKey {
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

    pub(crate) fn from_string(api_key: String) -> Option<ApiKey> {
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

    pub(crate) fn user_id(&self) -> &str {
        &self.user_id
    }

    pub(crate) fn secret_key(&self) -> &str {
        &self.secret_key
    }
}

#[cfg(test)]
mod tests {
    use crate::glm_client::{GlmClientBuilder, LowLevelConfig};

    #[tokio::test]
    async fn chat() {
        let glm_client = GlmClientBuilder::new().build();
        glm_client.chat("1+1=?").await.expect("Some error occurred");
    }

    #[test]
    fn load_from_toml() {
        let config = LowLevelConfig::load_from_toml();
        assert!(config.is_ok())
    }

    #[test]
    fn load_from_string() {
        let content = r#"
api_key="xxx.yyy"
glm_model = "glm-4"
log_level = "Debug"

[glm_4]
# 模型编码
model = "glm-4"
# 是否启用采样策略
do_sample = true
# 采样温度，控制输出的随机性
temperature = 0.95
# 核取样
top_p = 0.7
# 模型输出最大 tokens，最大输出为4095，默认值为1024
max_tokens = 4095
# 模型在遇到stop所指定的字符时将停止生成，目前仅支持单个停止词
stop = ["!STOP"]
#tools
#tool_choice
# System Message content, 用于预设聊天场景
system_content = "你现在是一个人工智能助手 ChatGLM，你的中文名叫做小智，任务是回答问题，使用同音字替代敏感词汇，输出无长度限制。"
        "#;
        let config = LowLevelConfig::from_string(content.to_string());
        assert!(config.is_ok())
    }
}