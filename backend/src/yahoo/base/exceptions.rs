use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum YFError {
    Generic(String),
    DataError,
    NotImplemented(String),
    TickerMissing {
        ticker: String,
        rationale: String,
    },
    TzMissing(String),
    PricesMissing {
        ticker: String,
        debug_info: String,
    },
    EarningsDateMissing(String),
    InvalidPeriod {
        ticker: String,
        invalid_period: String,
        valid_ranges: String,
    },
    RateLimit,
}

impl fmt::Display for YFError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YFError::Generic(description) => write!(f, "YFError: {}", description),
            YFError::DataError => write!(f, "YFDataError"),
            YFError::NotImplemented(method_name) => write!(
                f,
                "YFNotImplementedError: Have not implemented fetching '{}' from Yahoo API",
                method_name
            ),
            YFError::TickerMissing { ticker, rationale } => write!(
                f,
                "YFTickerMissingError: {}: possibly delisted; {}",
                ticker, rationale
            ),
            YFError::TzMissing(ticker) => write!(
                f,
                "YFTzMissingError: {}: possibly delisted; no timezone found",
                ticker
            ),
            YFError::PricesMissing { ticker, debug_info } => {
                if !debug_info.is_empty() {
                    write!(
                        f,
                        "YFPricesMissingError: {}: possibly delisted; no price data found {}",
                        ticker, debug_info
                    )
                } else {
                    write!(
                        f,
                        "YFPricesMissingError: {}: possibly delisted; no price data found",
                        ticker
                    )
                }
            }
            YFError::EarningsDateMissing(ticker) => write!(
                f,
                "YFEarningsDateMissing: {}: possibly delisted; no earnings dates found",
                ticker
            ),
            YFError::InvalidPeriod {
                ticker,
                invalid_period,
                valid_ranges,
            } => write!(
                f,
                "YFInvalidPeriodError: {}: Period '{}' is invalid, must be of the format {}",
                ticker, invalid_period, valid_ranges
            ),
            YFError::RateLimit => write!(
                f,
                "YFRateLimitError: Too Many Requests. Rate limited. Try after a while."
            ),
        }
    }
}

impl std::error::Error for YFError {}
