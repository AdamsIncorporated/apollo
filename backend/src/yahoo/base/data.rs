use crate::yahoo::constants::user_agents::USER_AGENTS;
use crate::yahoo::base::logger::Logger;
use reqwest::Client;
use std::sync::{Mutex, MutexGuard};
use rand::prelude::*;

use super::logger;

const CACH_MAXSIZE: u8 = 64;

pub struct YfData {
    pub user_agent_headers: String,
    session: Option<Client>,
    crumb: Option<String>,
    cookie: Option<String>,
    cookie_strategy: String,
    cookie_lock: Mutex<String>,
}

impl YfData {
    pub fn new(&self, session: Option<Client>) {
        let mut rng = rand::rng();

        YfData {
            user_agent_headers: USER_AGENTS
                .choose(&mut rng)
                .unwrap_or(&USER_AGENTS[0])
                .to_string(),
            session,
            crumb: None,
            cookie: None,
            cookie_strategy: "basic".to_string(),
            cookie_lock: Mutex::new(String::new()),
        };
    }

    fn set_session(&mut self, session: Option<Client>) {
        if !session.is_none() {
            self.session = session
        }
    }

    pub fn set_cookie_strategy(&mut self, strategy: String, have_lock: bool) {
        let logger = logger::Logger::instance();

        if strategy == self.cookie_strategy {
            return;
        }

        let mut lock_guard: Option<MutexGuard<String>> = None;

        if !have_lock {
            lock_guard = Some(self.cookie_lock.lock().unwrap());
        }

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.cookie_strategy == "csrf" {
                if let Some(session) = self.session.as_mut() {
                };
                logger.log(&format!(
                    "toggling cookie strategy {} -> basic",
                    self.cookie_strategy
                ));
                self.cookie_strategy = "basic".to_string();
            } else {
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
