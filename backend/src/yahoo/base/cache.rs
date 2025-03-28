use std::fmt;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread;

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

pub const CACH_INIT_LOCK: CachLock<i32> = CachLock::new(42);

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

struct CookieCacheManager {
    cookie_cache: Option<String>,
}

impl CookieCacheManager {
    pub fn new(&self) -> Self {
        CookieCacheManager { cookie_cache: None }
    }

    pub fn get_cookie_cache(&self) {
        if self.cookie_cache.is_none() {
        }
    }
}
