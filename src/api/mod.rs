pub mod members;
pub mod organizations;

use async_trait::async_trait;
use std::{error::Error, marker::PhantomData};

const ENDPOINT: &str = "https://api.trello.com/1";

pub struct Api {
    key: String,
    token: String,
}

pub enum RequestProtocol {
    GET,
    POST,
}

pub struct Request<Response>
where
    Response: serde::de::DeserializeOwned,
{
    _resp: PhantomData<Response>,
    url: String,
    protocol: RequestProtocol,
}

impl<Response> Request<Response>
where
    Response: serde::de::DeserializeOwned,
{
    fn new(url: String, protocol: RequestProtocol) -> Self {
        Self {
            _resp: PhantomData,
            url,
            protocol,
        }
    }

    pub fn get(url: String) -> Self {
        Self::new(url, RequestProtocol::GET)
    }

    pub async fn send(self) -> Result<Response, Box<dyn Error + Send + Sync>> {
        let body = reqwest::get(self.url).await?.text().await?;
        let response: Response = serde_json::from_str(body.as_str())?;
        Ok(response)
    }
}

impl Api {
    pub fn new(api_key: String) -> Self {
        Self {
            key: api_key,
            token: String::default(),
        }
    }

    pub fn auth(&mut self, token: String) {
        self.token = token;
    }

    fn get_req<Response>(&self, url: String) -> Request<Response>
    where
        Response: serde::de::DeserializeOwned,
    {
        Request::get(url)
    }
}
