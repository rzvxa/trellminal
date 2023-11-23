use super::{Api, Request, RequestFields, ENDPOINT};
use crate::models::{Board, Organization, OrganizationId};
use const_format::formatcp;

const URL_BASE: &str = formatcp!("{}/organizations", ENDPOINT);
const URL_BOARDS: &str = "boards";

pub trait Organizations {
    fn organizations_get(&self, id: OrganizationId) -> Request<Organization>;

    fn organizations_boards(&self, id: OrganizationId) -> Request<Vec<Board>>;
}

impl Organizations for Api {
    fn organizations_get(&self, id: OrganizationId) -> Request<Organization> {
        let fetch_user_url = format!("{}/{}", URL_BASE, id);
        self.get_req(
            fetch_user_url,
            RequestFields::List(vec!["id", "name", "displayName", "desc", "url", "teamType"]),
        )
    }

    fn organizations_boards(&self, id: OrganizationId) -> Request<Vec<Board>> {
        let fetch_user_url = format!("{}/{}/{}", URL_BASE, id, URL_BOARDS);
        self.get_req(
            fetch_user_url,
            RequestFields::List(vec!["id", "name", "desc", "url", "pinned", "starred"]),
        )
    }
}
