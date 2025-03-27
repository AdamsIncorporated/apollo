use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

struct Logger {
    file: Mutex<File>,
}

pub impl Logger {
    fn new(filename: &str) -> Result<Logger, std::io::Error> {
        let file = File::create(filename)?;
        Ok(Logger {
            file: Mutex::new(file),
        })
    }

    fn log(&self, message: &str) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let log_message = format!("[{}] {}\n", timestamp, message);

        if let Ok(mut file) = self.file.lock() {
            if let Err(e) = file.write_all(log_message.as_bytes()) {
                eprintln!("Error writing to log file: {}", e);
            }
        } else {
            eprintln!("Failed to acquire lock on log file.");
        }
    }
}
