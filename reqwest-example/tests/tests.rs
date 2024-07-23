
#[tokio::test]
async fn usage() {
    async fn request_get() -> Result<String, reqwest::Error> {
        let body = reqwest::get("https://www.rust-lang.org").await?
            .text().await?;
        Ok(body)
    }

    let result = request_get().await;
    assert!(result.is_ok());
    println!("result: {:?}", result.unwrap());
}
