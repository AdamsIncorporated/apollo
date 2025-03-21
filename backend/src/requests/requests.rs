use reqwest::header::{HeaderMap, HeaderValue};
use std::error::Error;
use serde_json::Value;
use urlencoding::encode;

pub async fn extract_financial_data() -> Result<Value, Box<dyn Error>> {
    let metrics = "annualTaxEffectOfUnusualItems,trailingTaxEffectOfUnusualItems,..."; // Your long metric string
    let encoded_metrics = encode(metrics);
    let url = format!("https://query1.finance.yahoo.com/ws/fundamentals-timeseries/v1/finance/timeseries/TSLA?merge=false&padTimeSeries=true&period1=493590046&period2=1742522399&type={}&lang=en-US&region=US", encoded_metrics);
    let mut headers = HeaderMap::new();
    headers.insert(
        "User-Agent",
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0",
        ),
    );
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert(
        "Accept-Language",
        HeaderValue::from_static("en-US,en;q=0.5"),
    );
    headers.insert(
        "Accept-Encoding",
        HeaderValue::from_static("gzip, deflate, br, zstd"),
    );
    headers.insert(
        "Referer",
        HeaderValue::from_static("https://finance.yahoo.com/quote/TSLA/financials/"),
    );
    headers.insert(
        "Origin",
        HeaderValue::from_static("https://finance.yahoo.com"),
    );
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-site"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let json: Value = response.json().await?;
        Ok(json)
    } else {
        Err(format!("HTTP request failed with status: {}", response.status()).into())
    }
}
