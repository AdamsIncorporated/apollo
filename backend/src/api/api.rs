use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

pub async fn fetch_data(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let user_agent = env::var("USER_AGENT").map_err(|_| "USER_AGENT environment variable not set")?;
    let user_agent_header = HeaderValue::from_str(&user_agent)?;

    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", user_agent_header);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("Status: {}", resp.status());
                println!("Headers:\n{:?}", resp.headers());

                let body_result = resp.text().await;
                match body_result {
                    Ok(body) => println!("Body:\n{}", body),
                    Err(e) => println!("Error reading body: {}", e),
                }
            } else {
                println!("Request failed with status: {}", resp.status());
                let error_body_result = resp.text().await;
                match error_body_result {
                    Ok(error_body) => println!("Error Body:\n{}", error_body),
                    Err(_) => println!("Could not retrieve error body."),
                }
            }
        }
        Err(e) => {
            println!("Request error: {}", e);
        }
    }

    Ok(())
}
