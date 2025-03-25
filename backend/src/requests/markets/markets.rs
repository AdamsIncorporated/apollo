use reqwest::Error as ReqwestError;
use crate::requests::auth::client::create_yahoo_finance_client;
use serde_json::{Value as JsonValue, Error as SerdeJsonError};

#[derive(Debug)]
pub enum CustomYahooFinanceError {
    Reqwest(ReqwestError),
    SerdeJson(SerdeJsonError),
}

impl std::fmt::Display for CustomYahooFinanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomYahooFinanceError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            CustomYahooFinanceError::SerdeJson(e) => write!(f, "Serde JSON error: {}", e),
        }
    }
}

impl std::error::Error for CustomYahooFinanceError {}

impl From<ReqwestError> for CustomYahooFinanceError {
    fn from(err: ReqwestError) -> Self {
        CustomYahooFinanceError::Reqwest(err)
    }
}

impl From<SerdeJsonError> for CustomYahooFinanceError {
    fn from(err: SerdeJsonError) -> Self {
        CustomYahooFinanceError::SerdeJson(err)
    }
}

pub async fn fetch_market_data() -> Result<JsonValue, CustomYahooFinanceError> {
    let client = create_yahoo_finance_client().await?;
    const BASE_URL: &str = "https://finance.yahoo.com/bonds";
    let response = client.get(BASE_URL).send().await?;
    let text = response.text().await?;
    println!("Text: {}", text);
    match serde_json::from_str(&text) {
        Ok(json) => Ok(json),
        Err(error) => Err(CustomYahooFinanceError::SerdeJson(error)),
    }
}