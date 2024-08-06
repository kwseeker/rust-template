use std::error::Error;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::runtime::Runtime;

/// 某个代码发现 serde_json 解析失败，但是没有抛出详细错误信息，不清楚哪里把错误信息给吞了
/// 结论只是
#[test]
fn test_handle_error() {
    #[derive(Debug)]
    pub(crate) struct PullRequestDiffs {
        diffs: Vec<PullRequestDiff>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub(crate) struct PullRequestDiff {
        /// 文件新增的行数
        additions: u32,
        /// 文件路径
        blob_url: String,
        /// 修改的行数
        changes: u32,
        contents_url: String,
        /// 删除的行数
        deletions: u32,
        /// 文件名
        filename: String,
        /// Patch 信息，即修改的内容
        patch: String,
        raw_url: String,
        sha: String,
        status: Option<Status>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    enum Status {
        Added,
        Modified,
        Removed,
        // Renamed,    // 原因是少定义了这一种类型
    }

    async fn convert(value: Value) -> anyhow::Result<PullRequestDiffs> {
        let diffs: Vec<PullRequestDiff> = serde_json::value::from_value(value)
            .context("Parse json body to PullRequestDiffs object failed")?;
            // .context("Parse json body to PullRequestDiffs object failed").unwrap();
        Ok(PullRequestDiffs{
            diffs
        })
    }

    fn convert2(value: Value) -> anyhow::Result<()> {
        let future = convert(value);
        let diffs = Runtime::new().unwrap().block_on(future)?;
        // let diffs = Runtime::new().unwrap().block_on(future).unwrap();
        println!("{diffs:?}");
        Ok(())
    }

    let value: Value = serde_json::from_str(JSON_ARRAY_STR)
        .expect("JSON was not well-formatted");

    let result = convert2(value);
    // result.unwrap();
    // 验证上面每一步，将 ? 使用 unwrap() 替代都可以打印详细错误信息
    // 那问题应该就是出现在这个 match 块中了
    match result {
        Ok(_) => println!("done"),
        Err(mut err) => {
            // panic!("Error: {err:?}");       //可以打印错误详细信息
            // eprintln!("Error: {:?}", err);       //无法打印错误详细信息
            // println!("Error: {:?}", err);        //无法打印错误详细信息
            if let Some(source) = err.source() {    //通过 Error 接口方法获取错误详细信息
                println!("Caused by: {:?}", source);
            }
            if let Some(cause) = err.cause() {    //通过 Error 接口方法获取错误详细信息. 接口已经废弃，使用 source() 代替
                println!("Caused by: {:?}", cause);
            }
        }
    }
}

const JSON_ARRAY_STR: &str = r#"[
{
    "sha": "4a30d2bdbe1aae2fe3ab00ba4c51a1ce13fb5d29",
    "filename": "ai-code-reviewer/crates/core/main.rs",
    "status": "renamed",
    "additions": 1,
    "deletions": 1,
    "changes": 2,
    "blob_url": "https://github.com/kwseeker/rust-template/blob/e15d5f98e5cb23db312a6fe58237656087785f30/ai-code-reviewer%2Fcrates%2Fcore%2Fmain.rs",
    "raw_url": "https://github.com/kwseeker/rust-template/raw/e15d5f98e5cb23db312a6fe58237656087785f30/ai-code-reviewer%2Fcrates%2Fcore%2Fmain.rs",
    "contents_url": "https://api.github.com/repos/kwseeker/rust-template/contents/ai-code-reviewer%2Fcrates%2Fcore%2Fmain.rs?ref=e15d5f98e5cb23db312a6fe58237656087785f30",
    "patch": "@@ -4,7 +4,7 @@ mod reviewer;\n mod openai;\n mod github;\n mod options;\n-mod errors;\n+mod common;\n \n /// ai-code-reviewer [OPTION]\n /// OPTIONS:",
    "previous_filename": "ai-code-reviewer/src/main.rs"
  }
]"#;