/// Github 客户端
struct GithubClient {

}

impl GithubClient {
    /// 读取 PR 信息
    fn list_pull_requests(&self) {

    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use reqwest::{Client, Method};
    use reqwest::header::{HeaderMap, HeaderValue};
    use serde_json;

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

    /// 请求 Github API， 测试时需要配置 GITHUB_TOKEN 环境变量
    async fn request_github(client: Client, method: Method, url: String, header_map: Option<HeaderMap>)
        -> Result<(), Box<dyn Error>>
    {
        let mut headers = if let Some(mut headers) = header_map {
            headers
        } else {
            HeaderMap::new()
        };
        // 添加必要的 header
        headers.insert("Accept", HeaderValue::from_static("application/vnd.github+json"));
        let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
        let final_token = format!("Bearer {}", token);
        match HeaderValue::from_str(&final_token) {
            Ok(token) => {
                headers.insert("Authorization", token);
            }
            Err(err) => {
                return Err(Box::new(err));
            }
        }
        headers.insert("X-GitHub-Api-Version", HeaderValue::from_static("2022-11-28"));
        headers.insert("User-Agent", HeaderValue::from_static("AI-Code-Reviewer")); // User-Agent 必须加，否则会 403 Forbidden

        let request_builder = client
            .request(method, url)
            .headers(headers);
        let res = request_builder.send().await;

        match res {
            Ok(response) => {
                if !response.status().is_success() {
                    println!("Request failed with status code: {}", response.status());
                    let error_text = response.text().await.unwrap_or_else(|_| "Failed to read error text".into());
                    println!("Error details: {}", error_text);
                } else {
                    println!("Request succeeded!");
                    let json_body: serde_json::Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
                    println!("Result: {}", serde_json::to_string_pretty(&json_body).unwrap());
                }
            },
            Err(e) => println!("Request failed with error: {}", e),
        }
        Ok(())
    }
}
