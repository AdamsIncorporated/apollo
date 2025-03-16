use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteResponse {
    pub quoteResponse: QuoteResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteResponseData {
    pub result: Vec<ResultItem>,
    pub error: Option<serde_json::Value>, // Can be null, so Option<serde_json::Value>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultItem {
    pub fullExchangeName: String,
    pub symbol: String,
    pub gmtOffSetMilliseconds: i64,
    pub language: String,
    pub regularMarketTime: RawFmt,
    pub regularMarketChangePercent: RawFmt,
    pub quoteType: String,
    pub typeDisp: String,
    pub tradeable: bool,
    pub regularMarketPreviousClose: RawFmt,
    pub exchangeTimezoneName: String,
    pub regularMarketChange: RawFmt,
    pub cryptoTradeable: bool,
    pub exchangeDataDelayedBy: i64,
    pub firstTradeDateMilliseconds: i64,
    pub exchangeTimezoneShortName: String,
    pub hasPrePostMarketData: bool,
    pub marketState: String,
    pub regularMarketPrice: RawFmt,
    pub customPriceAlertConfidence: String,
    pub market: String,
    pub quoteSourceName: String,
    pub priceHint: i64,
    pub exchange: String,
    pub sourceInterval: i64,
    pub shortName: String,
    pub region: String,
    pub triggerable: bool,
    pub longName: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawFmt {
    pub raw: f64,
    pub fmt: String,
}
