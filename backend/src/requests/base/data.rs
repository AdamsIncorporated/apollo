use crate::constants::USER_AGENTS;
use lru;
use rand::Rng;
use reqwest::Client;
use std::sync::{Mutex, OnceLock};

const CACH_MAXSIZE: int8 = 64;

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
            cookie_lock: Mutex<String>,
        }
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

        try { self.session.ca }
    }

    fn set_cookie_strategy(&self, strategy: &str, have_lock: Option<false>) {
        if (strategy == self.cookie_strategy) {
            return;
        } 

        if (!have_lock) {
            self.cookie_lock.
        }
    }
}
