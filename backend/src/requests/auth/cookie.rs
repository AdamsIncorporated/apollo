use reqwest::{Client, Error};

pub async fn get_yahoo_finance_with_cookies() -> Result<(), Error> {
    let client = Client::builder()
        .cookie_store(true)
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;

    let response = client.get("https://finance.yahoo.com/").send().await?;

    if response.status().is_success() {
        println!("Yahoo Finance request successful! Cookies handled.");
    } else {
        println!("Yahoo Finance request failed: {}", response.status());
    }

    Ok(())
}
