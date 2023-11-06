pub mod members;

const ENDPOINT: &str = "https://api.trello.com/1";

pub struct Api {
    key: String,
    token: String,
}

impl Api {
    pub fn new(api_key: String) -> Self {
        Self { key: api_key, token: String::default() }
    }

    pub fn auth(&mut self, token: String) {
        self.token = token;
    }
}
