use dirs_next::cache_dir;
use once_cell::sync::OnceCell;
use rusqlite::{Connection, Result as RusqliteResult};
use std::fmt;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};

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

    pub fn get_db(&mut self) {
        if let Some(conn) = self.db.take() {
            conn.close();
        }
    }

    pub fn initialise(&mut self) {
        if self.initialized != -1 {
            return;
        }

        let db = self.get_db();
    }

    pub fn lookup(&mut self, stratgey: String) {
        if self.dummy {
            return;
        }

        if self.initialized == -1 {
            self.initialise();
        }

        if self.initialized == 0 {
            return;
        }
    }

    pub fn store(&mut self, stratgey: String, cookie: String) {
        if self.dummy {
            return;
        }

        if self.initialized == -1 {
            self.initialise();
        }

        if self.initialized == 0 {
            return;
        }

        let db = self.get_db();

    }
}

pub struct CookieDBManager {
    db: Option<Connection>,
    cache_dir: PathBuf,
}

impl CookieDBManager {
    pub fn new() -> Self {
        let cache_dir = cache_dir()
            .map(|p| p.join("py-yfinance"))
            .unwrap_or_else(|| PathBuf::from("py-yfinance"));
        CookieDBManager {
            db: None,
            cache_dir: cache_dir,
        }
    }

    pub fn get_database() -> &'static Mutex<CookieDBManager> {
        static INSTANCE: OnceCell<Mutex<CookieDBManager>> = OnceCell::new();
        INSTANCE.get_or_init(|| Mutex::new(CookieDBManager::new()))
    }
}

pub struct TzDbManager {
    db: Option<Connection>,
    cach_dir: String,
}

impl TzDbManager {
    fn get_database() {}

    fn close_database(&mut self) -> RusqliteResult<()> {
        if let Some(conn) = self.db.take() {
            conn.close();
            Ok(())
        } else {
            Ok(())
        }
    }
}
