use std::cmp::PartialEq;
use std::time::{Duration, Instant};
use anyhow::{bail, Context};
use regex::Regex;
use reqwest::{Client, Method, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// Github 客户端
pub(crate) struct Github {
    client: Client,
    // pr_info: Option<PullRequestInfo>,
}

impl Github {
    pub(crate) fn new() -> Self {
        Github {
            client: Client::new(),
            // pr_info: None,
        }
    }

    // /// 通过 pr_number 读取 PR 信息
    // pub(crate) async fn get_pr_info(&self, pr_number: &usize) -> anyhow::Result<PullRequestInfo> {
    //     let url = format!("https://api.github.com/repos/kwseeker/rust-template/pulls/{}", pr_number);
    //     let url_cloned = url.clone();
    //     let res = http_request(&self.client, Method::GET, url, None).await;
    //     match res {
    //         Ok(response) => {
    //             return if response.status().is_success() {
    //                 println!("Request {} succeeded!", url_cloned);
    //                 let json_body: serde_json::Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    //                 // println!("Response json body {} succeeded!", serde_json::to_string_pretty(&json_body).unwrap());
    //                 let pr_info: PullRequestInfo = serde_json::value::from_value(json_body)
    //                     .context("Parse json body to PullRequestDiffs object failed")?;
    //                 Ok(pr_info)
    //             } else {
    //                 println!("Request {} failed with status code: {}", url_cloned, response.status());
    //                 let error_text = response.text().await.unwrap_or_else(|_| "Failed to read error text".into());
    //                 bail!(error_text)
    //             }
    //         },
    //         Err(e) => bail!("Request {} failed with error: {}", url_cloned, e),
    //     }
    // }

    /// 读取 PR diff 数据, 借助接口 /repos/{owner}/{repo}/pulls/{pull_number}/files
    pub(crate) async fn get_pr_diffs(&self, pr_number: &usize) -> anyhow::Result<PullRequestDiffs> {
        let url = format!("https://api.github.com/repos/kwseeker/rust-template/pulls/{}/files", pr_number);
        let url_cloned = url.clone();
        let res = http_request(&self.client, Method::GET, url, None).await;
        match res {
            Ok(response) => {
                return if response.status().is_success() {
                    println!("Request {} succeeded!", url_cloned);
                    let json_body: serde_json::Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
                    // println!("Response json body {} succeeded!", serde_json::to_string_pretty(&json_body).unwrap());
                    let diffs: Vec<PullRequestDiff> = serde_json::value::from_value(json_body)
                        .context("Parse json body to PullRequestDiffs object failed")?;
                    Ok(PullRequestDiffs {
                        diffs
                    })
                } else {
                    println!("Request {} failed with status code: {}", url_cloned, response.status());
                    let error_text = response.text().await.unwrap_or_else(|_| "Failed to read error text".into());
                    bail!(error_text)
                }
            }
            Err(e) => bail!("Request {} failed with error: {}", url_cloned, e),
        }
    }
}

/// HTTP 请求 Github API， 测试时需要配置 GITHUB_TOKEN 环境变量
async fn http_request(client: &Client, method: Method, url: String, header_map: Option<HeaderMap>)
                      -> anyhow::Result<Response>
{
    let now = Instant::now();
    let url_cloned = url.clone();

    // 添加必要的 header
    let mut headers = if let Some(headers) = header_map {
        headers
    } else {
        HeaderMap::new()
    };
    headers.insert("Accept", HeaderValue::from_static("application/vnd.github+json"));
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let final_token = format!("Bearer {}", token);
    let hv = HeaderValue::from_str(&final_token).context("Invalid header value for Authorization")?;
    headers.insert("Authorization", hv);
    headers.insert("X-GitHub-Api-Version", HeaderValue::from_static("2022-11-28"));
    headers.insert("User-Agent", HeaderValue::from_static("AI-Code-Reviewer")); // User-Agent 必须加，否则会 403 Forbidden

    let request_builder = client
        .request(method, url)
        .headers(headers);
    let res = request_builder
        .timeout(Duration::from_secs(60))   //墙内本地测试发现接口响应非常慢，配置代理没起作用，多给点时间
        .send().await;

    let elapsed = now.elapsed();
    println!("Time elapsed for request {url_cloned}: {:.2?}", elapsed);

    match res {
        Ok(response) => Ok(response),
        Err(err) => Err(anyhow::Error::from(err))
    }
}

// /// PR 信息
// #[derive(Debug, Default, Serialize, Deserialize)]
// #[serde(default)]
// struct PullRequestInfo {}

#[derive(Debug)]
pub(crate) struct PullRequestDiffs {
    diffs: Vec<PullRequestDiff>,
}

impl PullRequestDiffs {
    pub(crate) fn diffs(&self) -> &Vec<PullRequestDiff> {
        &self.diffs
    }
}

/// PR Diff 信息，每个对象对应一个文件
/// 详细参考 /repos/{owner}/{repo}/pulls/{pull_number}/files 接口返回值
#[derive(Debug, Default, Serialize, Deserialize)]
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

impl PullRequestDiff {
    /// 提取文件代码差异（主要在 patch 字段），过滤掉非代码文件、被删除的文件
    pub(crate) fn code_diffs(&self) -> anyhow::Result<Vec<String>> {
        // 过滤掉非代码文件、被删除的文件
        let mut need_review = match self.status {
            None => false,
            Some(Status::Added) => true,
            Some(Status::Modified) => true,
            Some(Status::Removed) => true,
        };
        need_review = need_review && CodeFile::is_code_file(&self.filename);
        if !need_review {
            return Ok(Vec::new());
        }

        // 每个patch中可能有多个差异块（diff block），拆分
        let regex = Regex::new(r"@@\s-(\d+)(?:,(\d+))?\s\+(\d+)(?:,(\d+))?\s@@")?;
        let mut diff_blocks = Vec::new();
        let (mut start, mut end) = (0, 0);
        loop {
            let mat = regex.find(&self.patch[start..]);
            if mat.is_none() {
                break;
            }
            // 新增行数为0，说明这个块中全是删除，不需要review
            let caps = regex.captures(mat.unwrap().as_str()).unwrap();
            let new_lines = caps.get(4).map_or(0, |m|
            m.as_str().parse::<usize>().unwrap_or(0));
            if new_lines <= 0 {
                break;
            }

            let range = mat.unwrap().range();
            end = range.end + start;
            let next_mat = regex.find(&self.patch[end..]);
            if next_mat.is_some() {
                let next_range = next_mat.unwrap().range();
                diff_blocks.push(self.patch[start..end + next_range.start].to_string());
                start = end + next_range.start;
            } else {
                diff_blocks.push(self.patch[start..].to_string());
                break;
            }
        }
        Ok(diff_blocks)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Status {
    Added,
    Modified,
    Removed,
}

enum CodeFile {
    C,
    Cpp,
    Go,
    Java,
    JavaScript,
    Lua,
    Python,
    Rust,
    TypeScript,
}

impl CodeFile {
    fn from_file_suffix(file_name: &String) -> Option<CodeFile> {
        let last_dot_idx = file_name.rfind(".");
        if last_dot_idx.is_none() {
            return None;
        }
        let p = last_dot_idx.unwrap() - 1;
        let file_suffix = &file_name[p..];
        match file_suffix {
            ".c" => Some(CodeFile::C),
            ".cpp" => Some(CodeFile::Cpp),
            ".go" => Some(CodeFile::Go),
            ".java" => Some(CodeFile::Java),
            ".js" => Some(CodeFile::JavaScript),
            ".lua" => Some(CodeFile::Lua),
            ".py" => Some(CodeFile::Python),
            ".rs" => Some(CodeFile::Rust),
            ".ts" => Some(CodeFile::TypeScript),
            _ => None,
        }
    }

    fn suffix(&self) -> &'static str {
        match self {
            CodeFile::C => ".c",
            CodeFile::Cpp => ".cpp",
            CodeFile::Go => ".go",
            CodeFile::Java => ".java",
            CodeFile::JavaScript => ".js",
            CodeFile::Lua => ".lua",
            CodeFile::Python => ".py",
            CodeFile::Rust => ".rs",
            CodeFile::TypeScript => ".ts",
        }
    }

    /// 从文件名后缀判断是否是代码文件
    fn is_code_file(file_name: &String) -> bool {
        Self::from_file_suffix(file_name).is_some()
    }
}

#[cfg(test)]
mod tests {
    use reqwest::{Client, Method};
    use reqwest::header::{HeaderMap};
    use serde_json;
    use crate::github::http_request;

    #[tokio::test]
    async fn get_pr_diffs() {
        let github = super::Github::new();
        let diffs = github.get_pr_diffs(&1).await.unwrap();
        assert!(diffs.diffs.len() > 0);
    }

    /// 读取所有 PR 请求
    /// curl -L \
    ///   -H "Accept: application/vnd.github+json" \
    ///   -H "Authorization: Bearer <YOUR-TOKEN>" \
    ///   -H "X-GitHub-Api-Version: 2022-11-28" \
    ///   https://api.github.com/repos/OWNER/REPO/pulls
    #[tokio::test]
    async fn list_pull_requests() {
        let client = Client::new();
        let url = "https://api.github.com/repos/kwseeker/rust-template/pulls";
        request_github(client, Method::GET, url.to_string(), None).await.unwrap();
    }

    #[tokio::test]
    async fn pull_request_info() {
        let client = Client::new();
        let pull_number = "1";
        let url = format!("https://api.github.com/repos/kwseeker/rust-template/pulls/{}", pull_number);
        request_github(client, Method::GET, url, None).await.unwrap();
    }

    #[tokio::test]
    async fn repo_content() {
        let client = Client::new();
        let url = "https://api.github.com/repos/kwseeker/rust-template/contents/.github";
        request_github(client, Method::GET, url.to_string(), None).await.unwrap();
    }

    #[tokio::test]
    async fn pull_request_files() {
        let client = Client::new();
        let url = "https://api.github.com/repos/kwseeker/rust-template/pulls/1/files";
        request_github(client, Method::GET, url.to_string(), None).await.unwrap();
    }

    /// 请求 Github API， 测试时需要配置 GITHUB_TOKEN 环境变量
    async fn request_github(client: Client, method: Method, url: String, header_map: Option<HeaderMap>)
                            -> anyhow::Result<()>
    {
        let res = http_request(&client, method, url, header_map).await;
        match res {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Request succeeded!");
                    let json_body: serde_json::Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
                    println!("Result: {}", serde_json::to_string_pretty(&json_body).unwrap());
                } else {
                    println!("Request failed with status code: {}", response.status());
                    let error_text = response.text().await.unwrap_or_else(|_| "Failed to read error text".into());
                    println!("Error details: {}", error_text);
                }
            }
            Err(e) => println!("Request failed with error: {}", e),
        }
        Ok(())
    }
}
