use crate::yahoo::constants;
use crate::yahoo::base::data::YfData;

pub struct TickerBase {
    pub ticker: String,
    pub proxy: String,
    pub session: String,
    tz: Option<String>,
    isin: Option<bool>,
    news: Option<Vec<String>>,
    shares: Option<Vec<String>>,
    earnings_dates: Option<Vec<String>>,
    financials: Option<Vec<String>>,
    data: YfData,
    price_history: Option<Vec<String>>,
    analysis: Option<String>,
    holders: Option<String>,
    quote: Option<String>,
    fundamentals: Option<String>,
    funds_data: Option<String>,
    fast_info: Option<String>,
}

#[derive(Clone)]
pub struct PriceHistory<'a> {
    data: &'a YfData,
    ticker: &'a Ticker,
    ticker_symbol: &'a TickerSymbol
}

#[derive(Clone)]
struct Ticker {

}
#[derive(Clone)]
struct TickerSymbol {

}

impl<'a> PriceHistory<'a> {
    fn new(data: &'a YfData, ticker: &'a Ticker, ticker_symbol: &'a TickerSymbol) -> Self {
        PriceHistory {
            data,
            ticker,
            ticker_symbol,
        }
    }
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
