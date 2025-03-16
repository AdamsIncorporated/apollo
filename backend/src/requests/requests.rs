use reqwest;
use serde_json::Value;
use std::error::Error;

pub async fn extract_financial_data(symbols: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!(
        "https://query1.finance.yahoo.com/v7/finance/quote?fields=longName,regularMarketPrice,regularMarketChange,regularMarketChangePercent,shortName,priceHint&formatted=true&imgHeights=50&imgLabels=logoUrl&imgWidths=50&symbols={}&lang=en-US&region=US&crumb=0b7HnC1AEdI",
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
