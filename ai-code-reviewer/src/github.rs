use std::time::{Duration, Instant};
use anyhow::{bail, Context};
use regex::Regex;
use reqwest::{Client, Method, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use crate::common::Null;

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
        let res = http_request::<Null>(&self.client, Method::GET, url, None, None).await;
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

    /// 创建 Review
    pub(crate) async fn create_review(&self, pr_number: &usize) -> anyhow::Result<()> {

        Ok(())
    }
}

/// HTTP 请求 Github API， 测试时需要配置 GITHUB_TOKEN 环境变量
async fn http_request<T>(client: &Client,
                         method: Method,
                         url: String,
                         header_map: Option<HeaderMap>,
                         body: Option<T>)
                         -> anyhow::Result<Response>
where
    T: Serialize,
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

    let mut request_builder = client
        .request(method, url)
        .headers(headers);
    if let Some(body) = body {
        request_builder = request_builder.json(&body);  //json() 中泛型约束是 Serialize + ?Sized，因为这里&body 取的引用，所以只需要 body 实现了 Serialize Trait
    }
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

    pub(crate) fn diffs_filtered(&self) -> Vec<PullRequestDiff> {
        let diffs: Vec<PullRequestDiff> = self.diffs.iter()
            .filter(|diff| diff.need_review())
            .cloned()
            .collect();
        // println!("diffs_filtered: {:?}", diffs);
        diffs
    }
}

/// PR Diff 信息，每个对象对应一个文件
/// 详细参考 /repos/{owner}/{repo}/pulls/{pull_number}/files 接口返回值
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

impl PullRequestDiff {
    /// 提取文件代码差异（主要在 patch 字段），过滤掉非代码文件、被删除的文件
    pub(crate) fn code_diffs(&self) -> anyhow::Result<Vec<String>> {
        // 过滤掉非代码文件、被删除的文件
        if !self.need_review() {
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
                let diff = self.patch[start..end + next_range.start].to_string();
                let code_diff = CodeDiff::new(self.filename.clone(), diff);
                diff_blocks.push(code_diff.to_string()?);
                start = end + next_range.start;
            } else {
                let diff = self.patch[start..].to_string();
                let code_diff = CodeDiff::new(self.filename.clone(), diff);
                diff_blocks.push(code_diff.to_string()?);
                break;
            }
        }
        Ok(diff_blocks)
    }

    fn need_review(&self) -> bool {
        let mut need_review = match self.status {
            None => false,
            Some(Status::Added) => true,
            Some(Status::Modified) => true,
            Some(Status::Removed) => false,
        };
        let need_review = need_review && Language::is_code_file(&self.filename);
        need_review
    }

    pub(crate) fn file_name(&self) -> &String {
        &self.filename
    }
}

/// 待评审的代码差异片段
#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct CodeDiff {
    filename: String,
    diff: String,
}

impl CodeDiff {
    pub(crate) fn new(filename: String, diff: String) -> Self {
        CodeDiff {
            filename,
            diff,
        }
    }

    pub(crate) fn to_string(&self) -> anyhow::Result<String> {
        let result = serde_json::to_string(self)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Status {
    Added,
    Modified,
    Removed,
}

enum Language {
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

impl Language {
    fn from_file_suffix(file_name: &String) -> Option<Language> {
        let last_dot_idx = file_name.rfind(".");
        if last_dot_idx.is_none() {
            return None;
        }
        let p = last_dot_idx.unwrap();
        let file_suffix = &file_name[p..];
        match file_suffix {
            ".c" => Some(Language::C),
            ".cpp" => Some(Language::Cpp),
            ".go" => Some(Language::Go),
            ".java" => Some(Language::Java),
            ".js" => Some(Language::JavaScript),
            ".lua" => Some(Language::Lua),
            ".py" => Some(Language::Python),
            ".rs" => Some(Language::Rust),
            ".ts" => Some(Language::TypeScript),
            _ => None,
        }
    }

    fn suffix(&self) -> &'static str {
        match self {
            Language::C => ".c",
            Language::Cpp => ".cpp",
            Language::Go => ".go",
            Language::Java => ".java",
            Language::JavaScript => ".js",
            Language::Lua => ".lua",
            Language::Python => ".py",
            Language::Rust => ".rs",
            Language::TypeScript => ".ts",
        }
    }

    /// 从文件名后缀判断是否是代码文件
    fn is_code_file(file_name: &String) -> bool {
        Self::from_file_suffix(file_name).is_some()
    }
}

#[derive(Serialize, Deserialize)]
struct ReviewBody {
    commit_id: String,
    body: String,
    event: ReviewEvent,
    comments: Vec<Comment>,
}

#[derive(Serialize, Deserialize)]
struct Comment {
    path: String,
    position: i32,
    body: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ReviewEvent {
    /// 只是评论
    Comment,
    /// 合并投票，一个PR一般需要达到最低投票数才能合并
    Approve,
    /// 必须修改，否则PR无法合并
    RequestChanges,
}

#[cfg(test)]
mod tests {
    use reqwest::{Client, Method};
    use reqwest::header::{HeaderMap};
    use serde::Serialize;
    use serde_json;
    use crate::common::Null;
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
        request_github_nobody(client, Method::GET, url.to_string(), None).await.unwrap();
    }

    #[tokio::test]
    async fn pull_request_info() {
        let client = Client::new();
        let pull_number = "1";
        let url = format!("https://api.github.com/repos/kwseeker/rust-template/pulls/{}", pull_number);
        request_github_nobody(client, Method::GET, url, None).await.unwrap();
    }

    #[tokio::test]
    async fn repo_content() {
        let client = Client::new();
        let url = "https://api.github.com/repos/kwseeker/rust-template/contents/.github";
        request_github_nobody(client, Method::GET, url.to_string(), None).await.unwrap();
    }

    #[tokio::test]
    async fn pull_request_files() {
        let client = Client::new();
        let url = "https://api.github.com/repos/kwseeker/rust-template/pulls/1/files";
        request_github_nobody(client, Method::GET, url.to_string(), None).await.unwrap();
    }

    #[tokio::test]
    async fn list_review_comment() {
        let client = Client::new();
        let url = "https://api.github.com/repos/kwseeker/rust-template/pulls/3/comments";
        request_github_nobody(client, Method::GET, url.to_string(), None).await.unwrap();
    }

    /// 关于 Review 和 Review Comment:
    /// Review 指代某次代码审查提交、Review Comment 则指代码审查中的评论，比如一个PR 可以审查多次，每次可能审查多个代码块
    /// 代码审查的评论信息需要提交后其他人才能看到
    /// 对应手动提交代码审查的完整流程是：先添加 Review Comments, 然后创建（创建时会整合 Comments）并提交 Review,
    /// 如果是AI评审直接创建 Review 并提交，创建 Review 时直接将AI的评审 Comment 格式化加入 comments 参数即可，详细参考 GitHub API 文档。
    #[tokio::test]
    async fn list_reviews() {
        let client = Client::new();
        let url = "https://api.github.com/repos/kwseeker/rust-template/pulls/3/reviews";
        request_github_nobody(client, Method::GET, url.to_string(), None).await.unwrap();
    }

    #[tokio::test]
    async fn create_review() {
        let client = Client::new();
        let url = "https://api.github.com/repos/kwseeker/rust-template/pulls/3/reviews";
        request_github_nobody(client, Method::POST, url.to_string(), None).await.unwrap();
    }

    async fn request_github_nobody(client: Client, method: Method, url: String, header_map: Option<HeaderMap>)
                                      -> anyhow::Result<()> {
        request_github::<Null>(client, method, url, header_map, None).await
    }

    /// 请求 Github API， 测试时需要配置 GITHUB_TOKEN 环境变量
    async fn request_github<T>(client: Client, method: Method, url: String, header_map: Option<HeaderMap>,
                                      body: Option<T>)
                                      -> anyhow::Result<()>
    where
        T: Serialize,
    {
        let res = http_request(&client, method, url, header_map, body).await;
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
