use reqwest;
use std::error::Error;


pub async fn extract_financial_data(symbols: &str) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "https://query1.finance.yahoo.com/v7/finance/quote?fields=longName,regularMarketPrice,regularMarketChange,regularMarketChangePercent&symbols={}",
        symbols
    );

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let json: Value = response.json().await?;
        Ok(json)
    } else {
        Err(format!("HTTP request failed with status: {}", response.status()).into())
    }
}
