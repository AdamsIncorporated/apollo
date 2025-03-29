use log::{self, Log, Metadata, Record};
use std::sync::Mutex;

struct YFLogFormatter;

impl log::Log for YFLogFormatter {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.target().starts_with("yfinance")
    }

    fn log(&self, record: &Record) {
        println!("yfinance: {}", record.args());
    }

    fn flush(&self) {}
}

static mut YF_LOGGER: Mutex<Option<Box<dyn Log>>> = Mutex::new(None);
static mut YF_LOG_INDENTED: bool = false;

fn get_indented_logger(name: &str) -> Box<dyn Log> {
    struct IndentedLogger {
        name: String,
    }

    impl log::Log for IndentedLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.target().starts_with(&self.name)
        }

        fn log(&self, record: &Record) {
            println!("  {}: {}", self.name, record.args()); // Example indentation
        }

        fn flush(&self) {}
    }

    Box::new(IndentedLogger {
        name: name.to_string(),
    })
}

fn get_yf_logger() -> &'static dyn Log {
    unsafe {
        let mut logger = YF_LOGGER.lock().unwrap();

        if YF_LOG_INDENTED {
            *logger = Some(get_indented_logger("yfinance"));
        } else if logger.is_none() {
            *logger = Some(Box::new(YFLogFormatter));
        }

        if let Some(ref log) = *logger {
            log.as_ref()
        } else {
            // If the logger is still None, provide a default no-op logger.
            struct NoOpLogger;
            impl log::Log for NoOpLogger {
                fn enabled(&self, _metadata: &Metadata) -> bool {
                    false
                }
                fn log(&self, _record: &Record) {}
                fn flush(&self) {}
            }
            static NO_OP_LOGGER: NoOpLogger = NoOpLogger;
            &NO_OP_LOGGER
        }
    }
}