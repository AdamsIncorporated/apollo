use crate::requests::contants;
use crate::requests::data::YfData;

pub struct TickerBase {
    pub ticker: String,
    pub proxy: String,
    pub session: String,
    tz: Option<String>,
    isin: Option<bool>,
    news: Option<Vec>,
    shares: Option<Vec>,
    earnings: Option<Vec>,
    financials: Option<Vec>,
    data: YfData,
    price_history: Option<Vec>,
    analysis: Option<String>,
    holders: Option<String>,
    quote: Option<String>,
    fundamentals: Option<String>,
    funds_data: Option<String>,
    fast_info: Option<String>,
}
