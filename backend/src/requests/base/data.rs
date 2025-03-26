use lru;
use std::sync::{Mutex, OnceLock};
use crate::constants::USER_AGENTS;
use rand::Rng;

const CACH_MAXSIZE: int8 = 64;

pub struct YfData {
    pub user_agent_headers: String,
    session: Option<String>,
    crumb: Option<String>,
    cookie: Option<String>,
    cookie_strategy: String,
    cookie_lock: String,
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
            cookie_lock: String::new(),
        }
    }

    pub fn get_instance() -> &'static Mutex<YfData> {
        static INTANCE: OnceLock<Mutex<YfData>> = OnceLock::new();
        INSTANCE.get_or_init(|| Mutex::new(YfData::new(None)))
    }
}
