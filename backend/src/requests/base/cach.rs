use bincode;
use directories::UserDirs;
use rusqlite::{params, types::Value, Connection, Row};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
enum CacheError {
    DatabaseError(String),
    IOError(String),
    SerializationError(String),
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            CacheError::IOError(msg) => write!(f, "IO error: {}", msg),
            CacheError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl Error for CacheError {}

struct TzCache {
    db_path: String,
    db: Mutex<Option<Connection>>,
    dummy: bool,
}

impl TzCache {
    fn new(cache_dir: &str) -> Result<Self, CacheError> {
        let db_path = format!("{}/tkr-tz.db", cache_dir);
        let db = Mutex::new(None);
        let dummy = false;

        if !Path::new(cache_dir).is_dir() {
            if let Err(e) = fs::create_dir_all(cache_dir) {
                return Err(CacheError::IOError(format!(
                    "Error creating TzCache folder: '{}' reason: {}",
                    cache_dir, e
                )));
            }
        } else if !fs::metadata(cache_dir)
            .map(|m| m.permissions().readonly())
            .unwrap_or(true)
        {
            return Err(CacheError::IOError(format!(
                "Cannot read and write in TzCache folder: '{}'",
                cache_dir
            )));
        }

        Ok(TzCache { db_path, db, dummy })
    }

    fn get_db(&self) -> Result<Connection, CacheError> {
        let mut db_guard = self.db.lock().unwrap();
        if let Some(conn) = &*db_guard {
            return Ok(conn.clone());
        }

        let conn = Connection::open(&self.db_path)
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;
        conn.execute_pragma(rusqlite::pragma::Pragma::JournalMode(
            rusqlite::pragma::JournalMode::Wal,
        ))
        .map_err(|e| CacheError::DatabaseError(e.to_string()))?;
        conn.execute_pragma(rusqlite::pragma::Pragma::CacheSize(-64))
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS kv (key TEXT PRIMARY KEY, value TEXT)",
            params![],
        )
        .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        *db_guard = Some(conn.clone());
        Ok(conn)
    }

    fn lookup(&self, key: &str) -> Result<Option<String>, CacheError> {
        if self.dummy {
            return Ok(None);
        }

        let db = self.get_db()?;
        let mut stmt = db
            .prepare("SELECT value FROM kv WHERE key = ?")
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;
        let mut rows = stmt
            .query(params![key])
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        if let Some(row) = rows
            .next()
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?
        {
            let value: String = row
                .get(0)
                .map_err(|e| CacheError::DatabaseError(e.to_string()))?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn store(&self, key: &str, value: Option<&str>) -> Result<(), CacheError> {
        if self.dummy {
            return Ok(());
        }

        let db = self.get_db()?;
        if let Some(v) = value {
            db.execute(
                "INSERT OR REPLACE INTO kv (key, value) VALUES (?, ?)",
                params![key, v],
            )
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;
        } else {
            db.execute("DELETE FROM kv WHERE key = ?", params![key])
                .map_err(|e| CacheError::DatabaseError(e.to_string()))?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct CookieData {
    cookie: HashMap<String, String>,
    age: Duration,
}

struct CookieCache {
    db_path: String,
    db: Mutex<Option<Connection>>,
    dummy: bool,
}

impl CookieCache {
    fn new(cache_dir: &str) -> Result<Self, CacheError> {
        let db_path = format!("{}/cookies.db", cache_dir);
        let db = Mutex::new(None);
        let dummy = false;

        if !Path::new(cache_dir).is_dir() {
            if let Err(e) = fs::create_dir_all(cache_dir) {
                return Err(CacheError::IOError(format!(
                    "Error creating CookieCache folder: '{}' reason: {}",
                    cache_dir, e
                )));
            }
        } else if !fs::metadata(cache_dir)
            .map(|m| m.permissions().readonly())
            .unwrap_or(true)
        {
            return Err(CacheError::IOError(format!(
                "Cannot read and write in CookieCache folder: '{}'",
                cache_dir
            )));
        }

        Ok(CookieCache { db_path, db, dummy })
    }

    fn get_db(&self) -> Result<Connection, CacheError> {
        let mut db_guard = self.db.lock().unwrap();
        if let Some(conn) = &*db_guard {
            return Ok(conn.clone());
        }

        let conn = Connection::open(&self.db_path)
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;
        conn.execute_pragma(rusqlite::pragma::Pragma::JournalMode(
            rusqlite::pragma::JournalMode::Wal,
        ))
        .map_err(|e| CacheError::DatabaseError(e.to_string()))?;
        conn.execute_pragma(rusqlite::pragma::Pragma::CacheSize(-64))
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS cookies (strategy TEXT PRIMARY KEY, fetch_date INTEGER, cookie_bytes BLOB)",
            params![],
        )
        .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        *db_guard = Some(conn.clone());
        Ok(conn)
    }

    fn lookup(&self, strategy: &str) -> Result<Option<CookieData>, CacheError> {
        if self.dummy {
            return Ok(None);
        }

        let db = self.get_db()?;
        let mut stmt = db
            .prepare("SELECT fetch_date, cookie_bytes FROM cookies WHERE strategy = ?")
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;
        let mut rows = stmt
            .query(params![strategy])
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        if let Some(row) = rows
            .next()
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?
        {
            let fetch_date: i64 = row
                .get(0)
                .map_err(|e| CacheError::DatabaseError(e.to_string()))?;
            let cookie_bytes: Vec<u8> = row
                .get(1)
                .map_err(|e| CacheError::DatabaseError(e.to_string()))?;
            let cookie: HashMap<String, String> = bincode::deserialize(&cookie_bytes)
                .map_err(|e| CacheError::SerializationError(e.to_string()))?;

            let age = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                - Duration::from_secs(fetch_date as u64);
            Ok(Some(CookieData { cookie, age }))
        } else {
            Ok(None)
        }
    }
}
