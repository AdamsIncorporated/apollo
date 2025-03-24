use reqwest::{Client, Error};
use Json;

pub async fn fetch_market_data(client: Client) -> Result<Json, Error> {
    const BASE_URL: &str = "https://finance.yahoo.com/bonds";
    client.get(BASE_URL).send().await?;
    Ok(client)
}
