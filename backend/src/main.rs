use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // Use the headers in your request
    let client = reqwest::Client::new();
    let response = client
        .get("https://www.example.com")
        .headers(headers)
        .send()?;

    // Handle the response
    println!("Status: {}", response.status());
    println!("Headers:\n{:?}", response.headers());
    println!("Body:\n{}", response.text()?);

    Ok(())
}
