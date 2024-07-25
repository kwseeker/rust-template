use std::time::{Duration, Instant};
use anyhow::{bail, Context};
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
            },
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Status {
    Added,
    Modified,
    Removed,
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
            },
            Err(e) => println!("Request failed with error: {}", e),
        }
        Ok(())
    }
}
