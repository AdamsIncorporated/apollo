use reqwest;
use std::error::Error;


pub async fn extract_financial_data(symbols: &str) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "https://query1.finance.yahoo.com/v7/finance/quote?fields=longName,regularMarketPrice,regularMarketChange,regularMarketChangePercent&symbols={}",
        symbols
    );

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("{}", body);
        Ok(body)
    } else {
        println!("Response status: {}", response.status());
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get data. Status: {}", response.status()),
        )))
    }
}
