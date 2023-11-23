use super::{Api, Request, RequestFields, ENDPOINT};
use crate::models::User;
use const_format::formatcp;

const URL_BASE: &str = formatcp!("{}/members", ENDPOINT);
const URL_ME: &str = formatcp!("{}/me", URL_BASE);

pub trait Members {
    fn members_me(&self) -> Request<User>;
}

impl Members for Api {
    fn members_me(&self) -> Request<User> {
        self.get_req(
            URL_ME.to_string(),
            RequestFields::List(vec![
                "id",
                "username",
                "fullName",
                "idBoards",
                "idOrganizations",
            ]),
        )
    }
}
