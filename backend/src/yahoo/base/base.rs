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
    earnings_dates: Option<Vec>,
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

enum LazyLoadResult {
    Loaded(PriceHistory),
    AlreadyLoaded,
}

impl TickerBase {
    fn lazy_load_price_history(&self) -> LazyLoadResult {
        if self.price_history.is_none() {
            let price_history = PriceHistory::new(&self.data, &self.ticker, &self.get_ticker(&self.proxy));
            self.price_history = Some(price_history.clone());
            LazyLoadResult::Loaded(price_history)
        } else {
            LazyLoadResult::AlreadyLoaded;
        }
    }

    fn get_ticker(&self, proxy: &str, timeout: &int32) {
        if proxy.is_none() {
            proxy = &self.proxy
        } 

        if !self.tz.is_none() {
            return self.tz
        }

        let c = ca
    }
}
