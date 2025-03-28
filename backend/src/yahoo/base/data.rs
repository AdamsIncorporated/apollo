use crate::constants::USER_AGENTS;
use crate::logger;
use lru;
use rand::Rng;
use reqwest::Client;
use std::sync::{Mutex, OnceLock};

const CACH_MAXSIZE: int8 = 64;

#[derive(Clone)]
pub struct YfData {
    pub user_agent_headers: String,
    session: Option<Client>,
    crumb: Option<String>,
    cookie: Option<String>,
    cookie_strategy: String,
    cookie_lock: Mutex<String>,
}

pub impl YfData {
    pub fn new(&self, session: session) {
        let mut rng = rand::thread_rng();

        YfData {
            user_agent_headers: USER_AGENTS.choose(&mut rng).unwrap_or(USER_AGENTS[0]),
            session,
            crumb: None,
            cookie: None,
            cookie_strategy: "basic".to_string(),
            cookie_lock: Mutex::new(String::new()),
        };

        self.set_session(&session);
    }

    pub fn get_instance() -> &'static Mutex<YfData> {
        static INTANCE: OnceLock<Mutex<YfData>> = OnceLock::new();
        self.set_session(session);
        INSTANCE.get_or_init(|| Mutex::new(YfData::new(None)))
    }

    fn set_session(&self, session: session) {
        match session {
            Some(value) => self.session = session,
            None => return,
        }
    }

    pub fn set_cookie_strategy(&mut self, strategy: String, have_lock: bool) {
        if strategy == self.cookie_strategy {
            return;
        }

        let mut lock_guard: Option<MutexGuard<String>> = None;

        if !have_lock {
            lock_guard = Some(self.cookie_lock.lock().unwrap());
        }

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.cookie_strategy == "csrf" {
                // Assuming self.session.as_mut().unwrap().cookies().clear() is how you clear cookies in reqwest.
                if let Some(session) = self.session.as_mut() {
                    //session.cookies().clear();
                };
                utils::get_yf_logger().debug(format!(
                    "toggling cookie strategy {} -> basic",
                    self.cookie_strategy
                ));
                self.cookie_strategy = "basic".to_string();
            } else {
                utils::get_yf_logger().debug(format!(
                    "toggling cookie strategy {} -> csrf",
                    self.cookie_strategy
                ));
                self.cookie_strategy = "csrf".to_string();
            }
            self.cookie = None;
            self.crumb = None;
        }));

        if let Err(e) = result {
            if !have_lock {
                drop(lock_guard); // Explicitly release the lock if we acquired it.
            }
            std::panic::resume_unwind(e); // Re-throw the panic
        }

        if !have_lock {
            drop(lock_guard); // Explicitly release the lock.
        }
    }
}
