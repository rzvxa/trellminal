use super::{ENDPOINT, Api};
use crate::models::User;
use const_format::formatcp;
use std::error::Error;
use async_trait::async_trait;

const URL_BASE: &str = formatcp!("{}/members", ENDPOINT);
const URL_ME: &str = formatcp!("{}/me", URL_BASE);

#[async_trait]
pub trait Members {
    async fn members_me(&self) -> Result<User, Box<dyn Error>>;
}

#[async_trait]
impl Members for Api {
    async fn members_me(&self) -> Result<User, Box<dyn Error>> {
        let fetch_user_url = format!("{}/?key={}&token={}", URL_ME, self.key, self.token);
        let body = reqwest::get(fetch_user_url).await?.text().await?;
        let user = serde_json::from_str(body.as_str())?;
        Ok(user)
    }
}
