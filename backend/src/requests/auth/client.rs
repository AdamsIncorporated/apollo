use reqwest::{Client, Error};


pub async fn create_yahoo_finance_client() -> Result<Client, Error> {
    let client = Client::builder()
        .cookie_store(true)
        .build()?;

    const BASE_URL: &str = "https://finance.yahoo.com/";
    client.get(BASE_URL).send().await?;
    Ok(client)
}


