use reqwest::{Client, Error};


pub async fn create_yahoo_finance_client() -> Result<Client, Error> {
    let client = Client::builder()
        .cookie_store(true)
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;

    const BASE_URL: &str = "https://finance.yahoo.com/";
    client.get(BASE_URL).send().await?;
    Ok(client)
}
