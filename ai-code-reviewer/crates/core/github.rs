use std::ops::Add;
use std::time::{Duration, Instant};
use anyhow::{bail, Context};
use log::{debug, error};
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
                    debug!("Request {} succeeded!", url_cloned);
                    let response_text = response.text().await?;
                    let json_body: serde_json::Value = serde_json::from_str(&response_text).unwrap();
                    debug!("Response json body {} succeeded!", serde_json::to_string_pretty(&json_body).unwrap());
                    // let diffs: Vec<PullRequestDiff> = serde_json::value::from_value(json_body)
                    //     .context("Parse json body to PullRequestDiffs object failed")?;
                    let diffs: Vec<PullRequestDiff> = serde_json::from_str(&response_text)
                        .context("Parse json body to PullRequestDiffs object failed")?;
                    Ok(PullRequestDiffs {
                        diffs
                    })
                } else {
                    error!("Request {} failed with status code: {}", url_cloned, response.status());
                    let error_text = response.text().await.unwrap_or_else(|_| "Failed to read error text".into());
                    bail!(error_text)
                }
            }
            Err(e) => bail!("Request {} failed with error: {}", url_cloned, e),
        }
    }

    /// 通过接口实现代码 Review, 两种方式：
    /// 1）先创建 Pending 状态的 Review, 然后添加评审信息，最后提交
    /// 2）直接通过 Create Review API 中提交（选择 APPROVE, REQUEST_CHANGES, or COMMENT），AI 代码评审使用这种方式直接提交就行
    pub(crate) async fn create_review(&self, pr_number: &usize, comments: Vec<Comment>) -> anyhow::Result<()> {
        let url = format!("https://api.github.com/repos/kwseeker/rust-template/pulls/{}/reviews", pr_number);
        let url_cloned = url.clone();
        let current = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let body = Some(ReviewBody {
            commit_id: None,    // 让GitHub自动获取最新提交即可
            body: Some(String::from("AI code reviewer auto comments at ").add(&current)),
            event: Some(ReviewEvent::Comment),    //AI 评审仅仅作为 Comment 建议，不记录投票，也不要求强制修改
            comments,
        });
        let res = http_request::<ReviewBody>(&self.client, Method::POST, url, None, body).await;
        match res {
            Ok(response) => {
                return if response.status().is_success() {
                    debug!("Request {} succeeded!", url_cloned);
                    // let json_body: serde_json::Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
                    // debug!("Response json body {} succeeded!", serde_json::to_string_pretty(&json_body).unwrap());
                    Ok(())
                } else {
                    error!("Request {} failed with status code: {}", url_cloned, response.status());
                    let error_text = response.text().await.unwrap_or_else(|_| "Failed to read error text".into());
                    bail!(error_text)
                }
            }
            Err(e) => bail!("Request {} failed with error: {}", url_cloned, e),
        }
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
        debug!("Request body: {}", serde_json::to_string_pretty(&body).unwrap());
        request_builder = request_builder.json(&body);  //json() 中泛型约束是 Serialize + ?Sized，因为这里&body 取的引用，所以只需要 body 实现了 Serialize Trait
    }
    let res = request_builder
        .timeout(Duration::from_secs(60))   //墙内本地测试发现接口响应非常慢，配置代理没起作用，多给点时间
        .send().await;

    let elapsed = now.elapsed();
    debug!("Time elapsed for request {url_cloned}: {:.2?}", elapsed);

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
        // debug!("diffs_filtered: {:?}", diffs);
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
    /// 提取文件中代码差异（主要在 patch 字段），过滤掉非代码文件、被删除的文件
    pub(crate) fn code_diffs(&self) -> anyhow::Result<Vec<DiffHunk>> {
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
            let new_lines = caps.get(4).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
            if new_lines <= 0 {
                break;
            }

            let old_start = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let old_lines = caps.get(2).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
            let new_start = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();

            let range = mat.unwrap().range();
            end = range.end + start;
            let next_mat = regex.find(&self.patch[end..]);
            if next_mat.is_some() {
                let next_range = next_mat.unwrap().range();
                let diff = self.patch[start..end + next_range.start].to_string();
                let hunk_line = HunkLine::from_last_line(diff.clone(), old_start, old_lines, new_start, new_lines);
                let diff_hunk = DiffHunk::new(self.filename.clone(), diff, hunk_line);
                diff_blocks.push(diff_hunk);
                start = end + next_range.start;
            } else {
                let diff = self.patch[start..].to_string();
                let hunk_line = HunkLine::from_last_line(diff.clone(), old_start, old_lines, new_start, new_lines);
                let diff_hunk = DiffHunk::new(self.filename.clone(), diff, hunk_line);
                diff_blocks.push(diff_hunk);
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
            Some(Status::Renamed) => true,
        };
        let need_review = need_review && Language::is_code_file(&self.filename);
        need_review
    }

    pub(crate) fn file_name(&self) -> &String {
        &self.filename
    }
}

/// 待评审的代码差异片段（对应每一个 @@ hunk）
#[derive(Serialize, Deserialize)]
pub(crate) struct DiffHunk {
    /// 实际是文件相对于项目根目录的相对路径
    filename: String,
    /// @@ hunk 内容
    diff: String,
    /// hunk 最后一行，计划是将AI评审结果添加到 diff hunk 的最后一行后面
    last_line: HunkLine,
}

impl DiffHunk {
    pub(crate) fn new(filename: String, diff: String, hunk_line: HunkLine) -> Self {
        DiffHunk {
            filename,
            diff,
            last_line: hunk_line,
        }
    }

    pub(crate) fn to_string(&self) -> anyhow::Result<String> {
        let result = serde_json::to_string(self)?;
        Ok(result)
    }

    pub(crate) fn filename(&self) -> &String {
        &self.filename
    }


    pub(crate) fn last_line(&self) -> &HunkLine {
        &self.last_line
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct HunkLine {
    line: u32,
    side: Side,
}

impl HunkLine {
    // 获取 @@ hunk 最后一行的HunkLine
    fn from_last_line(mut hunk: String, old_start: u32, old_lines: u32, new_start: u32, new_lines:u32) -> HunkLine {
        // 如果包含文件末尾的话，如果没有留空行，Github 会为 @@ hunk 自动添加一个空行 “\n\\ No newline at end of file”
        let github_end_line = "\\ No newline at end of file";
        if hunk.ends_with(github_end_line) {
            hunk = hunk.trim_end_matches(github_end_line).to_string();
        }
        // 如果最后一行行尾为换行符，则去除
        hunk = hunk.trim_end_matches('\n').to_string();
        // 查找最后一行的开头
        let mut idx = hunk.rfind("\n");
        return match idx {
            None => {
                HunkLine {   //只有一行
                    line: new_start,
                    side: Side::Right,
                }
            }
            Some(index) => {
                let last_line = &hunk[index..];
                if last_line.starts_with("-") {
                    HunkLine {
                        line: old_start + old_lines - 1,
                        side: Side::Left,
                    }
                } else {
                    HunkLine {
                        line: new_start + new_lines - 1,
                        side: Side::Right,
                    }
                }
            }
        }
    }

    fn line(&self) -> u32 {
        self.line   //基本数据类型在栈上分配，实际返回的是一个副本
    }

    fn side(&self) -> &Side {
        &self.side
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Status {
    Added,
    Modified,
    Removed,
    Renamed,
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
pub(crate) struct ReviewBody {
    /// 差异代码所属提交的ID (SHA)，不指定的话取 PR 中最新的 commit ID
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_id: Option<String>,
    /// 文本字符串，对应手动操作时 Review changes 框中的信息
    /// 创建 REQUEST_CHANGES 或 COMMENT Review 时，必填
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    /// Review 行为类型，APPROVE, REQUEST_CHANGES, or COMMENT， 还有一种 Pending 中间状态
    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<ReviewEvent>,
    /// review comment 列表
    comments: Vec<Comment>,
}

/// 关于下面字段建议自行去 GitHub 测试下，有些字段官方文档说的也不是很清楚
#[derive(Serialize, Deserialize)]
pub(crate) struct Comment {
    /// 评论注释文件的相对路径
    path: String,
    /// 从官方文档翻译过来意思是
    /// 你想添加评论的行相对于第一个 "@@" hunk 头向下的行数，但是在 devtools 手动测试发现好像又不太对, TODO ?
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<i32>,
    /// 评论内容
    body: String,
    // 后面4个是可选字段，但是官方文档没有解释什么用途，有需要自行测试吧
    /// 推测是和 side 一起使用，表示想添加评论的行是旧文件的行还是新文件的行
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u32>,
    /// 推测是和 line 一起使用，side: Left 表示 line 是旧文件的行， right
    #[serde(skip_serializing_if = "Option::is_none")]
    side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_line: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_side: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
// #[serde(rename_all = "lowercase")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Side {
    Left,
    Right,
}

impl Comment {
    pub(crate) fn new(path: String, body: String) -> Self {
        Comment {
            path,
            position: None,
            body,
            line: None,
            side: None,
            start_line: None,
            start_side: None,
        }
    }

    pub(crate) fn new_with_line(path: &String, body: String, last_line: &HunkLine) -> Self {
        let mut comment = Comment::new(path.clone(), body);
        comment.line(last_line.line());
        comment.side(last_line.side().clone());
        comment
    }

    fn position(&mut self, position: i32) {
        self.position = Some(position)
    }

    fn line(&mut self, line: u32) {
        self.line = Some(line)
    }

    fn side(&mut self, side: Side) {
        self.side = Some(side)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ReviewEvent {
    /// 默认的行为，表示Review尚未完成，完成后需要提交，这只是 Review 的中间状态，提交时选择后面三种状态之一
    Pending,
    /// 只是评论，不参与投票计数
    Comment,
    /// 合并投票，一个PR一般需要达到最低投票数才能合并
    Approve,
    /// 必须修改，否则PR无法合并，不参与投票计数
    RequestChanges,
}

#[cfg(test)]
mod tests {
    use std::ops::Add;
    use log::{debug, error};
    use reqwest::{Client, Method};
    use reqwest::header::{HeaderMap};
    use serde::Serialize;
    use serde_json;
    use crate::common::Null;
    use crate::github::{Comment, http_request, ReviewBody, ReviewEvent, Side};

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

    /// 测试通过 API 创建 Review Comment
    #[tokio::test]
    async fn create_review() {
        let client = Client::new();
        let url = "https://api.github.com/repos/kwseeker/rust-template/pulls/3/reviews";
        // 获取当前时间 2023-01-01 00:00:00 格式
        let current = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let mut comments = Vec::new();
        // 每个diff段一个Comment
        let mut comment = Comment::new(
            "ai-code-reviewer/src/github.rs".to_string(),
            "AI's some comments here ...".to_string(),
        );
        // comment.position(96);
        comment.line(118);
        comment.side(Side::Right);
        comments.push(comment);
        let body = Some(ReviewBody {
            commit_id: Some("a8c502ecfdf14d705274d70bfd4f55885b4417dd".to_string()),
            body: Some(String::from("AI code reviewer auto comments at ").add(&current)),
            event: Some(ReviewEvent::Comment),    //AI 评审仅仅作为 Comment 建议
            comments,
        });
        request_github(client, Method::POST, url.to_string(), None, body).await.unwrap();
    }

    /// 测试 Create Review API 默认值
    #[tokio::test]
    async fn create_review_with_default_params() {
        let client = Client::new();
        let url = "https://api.github.com/repos/kwseeker/rust-template/pulls/3/reviews";
        // 获取当前时间 2023-01-01 00:00:00 格式
        let current = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let mut comments = Vec::new();
        // 每个diff段一个Comment
        let mut comment = Comment::new(
            "ai-code-reviewer/src/github.rs".to_string(),
            "AI's some comments here ...".to_string(),
        );
        comment.line(120);
        comment.side(Side::Right);
        comments.push(comment);
        let body = Some(ReviewBody {
            commit_id: None,
            body: Some(String::from("AI code reviewer auto comments at ").add(&current)),
            event: Some(ReviewEvent::Comment),    //AI 评审仅仅作为 Comment 建议
            comments,
        });
        request_github(client, Method::POST, url.to_string(), None, body).await.unwrap();
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
