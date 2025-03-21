use reqwest;
use serde_json::Value;
use std::error::Error;

pub async fn extract_financial_data(symbols: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!(
        "https://query1.finance.yahoo.com/v7/finance/quote?fields=longName,regularMarketPrice,regularMarketChange,regularMarketChangePercent,shortName,priceHint&formatted=true&imgHeights=50&imgLabels=logoUrl&imgWidths=50&symbols={}&lang=en-US&region=US&crumb=0b7HnC1AEdI",
        symbols
    );

    let response = reqwest::get(&url).await?;

    let text_result = response.text().await;

    match text_result {
        Ok(text) => {
            match serde_json::from_str(&text) {
                Ok(json_value) => {
                    let json: Value = json_value;
                    // Now you can work with the `json` Value
                    println!("Parsed JSON: {:?}", json);
                }
                Err(e) => {
                    eprintln!("Error parsing JSON: {}", e);
                    //Handle error.
                    return Ok(()); //or return Err(some_error);
                }
            }
        }
        Err(e) => {
            eprintln!("Error getting response text: {}", e);
            //Handle error.
            return Ok(); //or return Err(e);
        }
    }
    Ok()
}
