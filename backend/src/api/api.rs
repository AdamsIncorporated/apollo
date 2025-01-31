use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

pub async fn fetch_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let user_agent = env::var("USER_AGENT").map_err(|_| "USER_AGENT environment variable not set")?;
    let user_agent_header = HeaderValue::from_str(&user_agent)?;

    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", user_agent_header);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await?;

    let status = response.status();

    if status.is_success() {
        let body = response.text().await?;
        Ok(body)
    } else {
        let error_body = response.text().await.unwrap_or_default();
        Err(format!("Request failed with status: {} and body: {}", status, error_body).into())
    }
}