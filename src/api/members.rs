use super::{Api, Request, ENDPOINT};
use crate::models::User;
use const_format::formatcp;

const URL_BASE: &str = formatcp!("{}/members", ENDPOINT);
const URL_ME: &str = formatcp!("{}/me", URL_BASE);

pub trait Members {
    fn members_me(&self) -> Request<User>;
}

impl Members for Api {
    fn members_me(&self) -> Request<User> {
        let fetch_user_url = format!("{}/?key={}&token={}", URL_ME, self.key, self.token);
        self.get_req(fetch_user_url)
    }
}
