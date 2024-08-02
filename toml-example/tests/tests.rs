use serde::Deserialize;

#[test]
fn usage() {
    #[derive(Deserialize, Debug)]
    struct LowLevelConfig {
        api_key: Option<String>,
        glm_model: Option<String>,
        log_level: Option<String>,
        glm_4: Glm4Config,
    }

    #[derive(Deserialize, Debug)]
    struct Glm4Config {
        model: String,
        // #[serde(default)]
        do_sample: bool,
        // #[serde(default)]
        temperature: f32,
        // #[serde(default)]
        top_p: f32,
        // #[serde(default)]
        max_tokens: usize,
        stop: Vec<String>,
        system_content: Option<String>,
    }

    // 好像还不支持为不存在于toml文件中的字段设置默认值，而且现在要求struct中的字段在toml文件中的字段要么必须存在，要么是Option类型字段，不然会报错
    // impl Default for Glm4Config {
    //     fn default() -> Self {
    //         Glm4Config {
    //             model: "glm-4".to_string(),
    //             do_sample: true,
    //             temperature: 0.95,
    //             top_p: 0.7,
    //             max_tokens: 4095,
    //             stop: vec![],
    //             system_content: None,
    //         }
    //     }
    // }

    let str = r#"api_key="xxx.yyy"
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
# 模型在遇到stop所制定的字符时将停止生成，目前仅支持单个停止词
stop = ["!STOP"]
# tools
# tool_choice
# System Message content, 用于预设聊天场景
system_content = "你现在是一个人工智能助手 ChatGLM，你的中文名叫做小智，任务是回答问题，使用同音字替代敏感词汇，输出无长度限制。""#;
    let config: LowLevelConfig = toml::from_str(str).unwrap();
    println!("{:?}", config)
}