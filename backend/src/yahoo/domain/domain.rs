use std::collections::HashMap;

pub struct Domain {
    pub proxy: Option<String>,
    pub session: Option<String>,
    pub name: String,
    key: String,
    data: YfData,
    pub symbol: String,
    pub ticker: String,
    pub overview: HashMap<String, String>,
    pub top_companies: Vec<HashMap<String, String>>,
    pub research_reports: Vec<HashMap<String, String>>,
}

impl Domain {
    pub fn new(&self, key: &str, session: Option<String>, proxy: Option<String>) {
        
    }

    fn fetch(&self, query_url: &str, proxy: &str) -> HashMap<String, String> {
        let params_dict: HashMap<String, String> = [
            ("formatted".to_string(), "true".to_string()),
            ("withReturns".to_string(), "true".to_string()),
            ("lang".to_string(), "en-US".to_string()),
            ("region".to_string(), "US".to_string()),
        ]
        .iter()
        .cloned()
        .collect();

        let result = self.data.get_raw_json(
            query_url,
            &self.data.user_agent_headers,
            params_dict,
            proxy,
        )?;

        Ok(result)
    }
}
