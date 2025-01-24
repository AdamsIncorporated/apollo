use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut headers = HeaderMap::new();
    let user_agent: String = match env::var("USER_AGENT") {
        Ok(user) => user,
        Err(_) => {
            return Err("USER_AGENT environment variable not found.".into());
        }
    };
    let user_agent_header: HeaderValue = match HeaderValue::from_str(&user_agent) {
        Ok(key) => key,
        Err(error) => return Err(format!("Invalid USER_AGENT header value: {}", error).into()),
    };

    headers.insert("User-Agent", user_agent_header);

    let client = reqwest::Client::new();
    let response = client
        .get("https://www.example.com")
        .headers(headers)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("Status: {}", resp.status());
                println!("Headers:\n{:?}", resp.headers());

                match resp.text().await {
                    Ok(body) => println!("Body:\n{}", body),
                    Err(e) => println!("Error reading body: {}", e),
                }
            } else {
                // If the response has an error status
                println!("Request failed with status: {}", resp.status());
            }
        }
        Err(e) => {
            // If there was an error sending the request
            println!("Request error: {}", e);
        }
    }

    Ok(())
}
