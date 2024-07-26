# RustGLM 封装原理与AI接口使用

RustGLM 源码仅仅2K行，不熟悉与AI大模型交互流程的完全可以通读下源码理解下程序是怎么与ChatGLM进行交互的。

依赖项:

```yaml
[dependencies]
toml = "0.8.10"
chrono = "0.4.33"
hmac = "0.12.1"
sha2 = "0.10.8"
base64url = "0.1.0"
rsntp = "4.0.0"
once_cell = "1.19.0"
serde_json = "1.0.113"
reqwest = { version = "0.12.1", features = ["json", "blocking", "stream"] }
serde = { version = "1.0.196", features = ["derive"] }
regex = { version = "1.10.3"}
tokio = { version = "1.35.1", features = ["full"] }
time = "0.3.32"
lazy_static = "1.4.0"
anyhow = "1.0.79"
tokio-util = "0.7.10"
futures-util = { version = "0.3.30", features = ["compat"] }
futures = "0.3.30"
serde_derive = "1.0.197"
async-trait = "0.1.77"
```







