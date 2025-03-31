use std::fmt;
use std::sync::{Mutex, MutexGuard, OnceLock};
use rusqlite::{params, Connection, Result as RusqliteResult};

pub struct CachLock<T> {
    mutex: Mutex<T>,
}

impl<T> CachLock<T> {
    pub const fn new(value: T) -> Self {
        CachLock {
            mutex: Mutex::new(value),
        }
    }

    pub fn lock(&self) -> Result<MutexGuard<T>, std::sync::PoisonError<MutexGuard<T>>> {
        self.mutex.lock()
    }
}

#[derive(Debug)]
pub struct CookieCacheException {
    message: String,
}

impl CookieCacheException {
    fn new(message: &str) -> Self {
        CookieCacheException {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for CookieCacheException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CookieCacheException: {}", self.message)
    }
}

impl std::error::Error for CookieCacheException {}

static INSTANCE: OnceLock<Mutex<String>> = OnceLock::new();

pub struct CookieCacheManager {
    cookie_cache: Option<String>,
}

impl CookieCacheManager {
    pub fn new() -> Self {
        CookieCacheManager { cookie_cache: None }
    }

    pub fn get_cookie_cache(&self) -> &'static Mutex<String> {
        INSTANCE.get_or_init(|| Mutex::new(String::new()))
    }
}

pub struct CookieCache {
    pub initialized: i8,
    pub db: Option<Connection>,
    pub dummy: bool,
}

impl CookieCache {
    pub fn new() -> Self {
        // panic because I have no idea how this code works externally
        let conn = Connection::open_in_memory().unwrap();

        CookieCache {
            initialized: -1,
            db: Some(conn),
            dummy: false,
        }
    }

    pub fn get_db(&mut self) -> RusqliteResult<()> {
        if let Some(conn) = self.db.take() {
            conn.close();
            Ok(())
        } else {
            Ok(())
        }
    }
}


pub struct TzDbManager {
    db: Option<Connection>,
    cach_dir: String
}

impl TzDbManager {
    fn get_database() {

    }

    fn close_database(&self) {
        if !self.db.is_none() {
            self.db::Close();
        }
    }
}