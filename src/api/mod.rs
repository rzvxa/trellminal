pub mod members;
pub mod organizations;

use std::marker::PhantomData;
use thiserror::Error;

const ENDPOINT: &str = "https://api.trello.com/1";

pub struct Api {
    key: String,
    token: String,
}

#[derive(Error, Debug)]
pub enum SendRequestError {
    #[error("Token Expired")]
    ExpiredToken,
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
}

pub enum RequestFields<'a> {
    All,
    Raw(&'a str),
    List(Vec<&'a str>),
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

    pub async fn send(self) -> Result<Response, SendRequestError> {
        match self.protocol {
            RequestProtocol::GET => self.send_get().await,
            RequestProtocol::POST => self.send_post().await,
        }
    }

    async fn send_get(self) -> Result<Response, SendRequestError> {
        let body = reqwest::get(self.url).await?.text().await?;
        let body: &str = body.as_str();
        match body {
            "expired token" => Err(SendRequestError::ExpiredToken),
            _ => {
                let response: Response = serde_json::from_str(body)?;
                Ok(response)
            }
        }
    }

    async fn send_post(self) -> Result<Response, SendRequestError> {
        todo!()
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

    fn get_req<Response>(&self, url: String, fields: RequestFields) -> Request<Response>
    where
        Response: serde::de::DeserializeOwned,
    {
        let fields = match fields {
            RequestFields::All => "all".to_string(),
            RequestFields::Raw(fields) => fields.to_string(),
            RequestFields::List(fields) => fields.join(","),
        };
        let url = format!(
            "{}/?key={}&token={}&fields={}",
            url, self.key, self.token, fields
        );
        Request::get(url)
    }
}
