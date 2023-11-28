use super::{Api, Request, RequestFields, ENDPOINT};
use crate::models::{BoardId, Card, List};
use const_format::formatcp;

const URL_BASE: &str = formatcp!("{}/boards", ENDPOINT);
const URL_LISTS: &str = "lists";
const URL_CARDS: &str = "cards";

pub trait Boards {
    fn boards_lists(&self, id: &BoardId) -> Request<Vec<List>>;
    fn boards_cards(&self, id: &BoardId) -> Request<Vec<Card>>;
}

impl Boards for Api {
    fn boards_lists(&self, id: &BoardId) -> Request<Vec<List>> {
        let fetch_user_url = format!("{}/{}/{}", URL_BASE, id, URL_LISTS);
        self.get_req(
            fetch_user_url,
            RequestFields::List(vec!["id", "name", "idBoard", "idOrganization"]),
        )
    }

    fn boards_cards(&self, id: &BoardId) -> Request<Vec<Card>> {
        let fetch_user_url = format!("{}/{}/{}", URL_BASE, id, URL_CARDS);
        self.get_req(
            fetch_user_url,
            RequestFields::List(vec![
                "id",
                "name",
                "idBoard",
                "idOrganization",
                "idList",
                "labels",
                "url",
            ]),
        )
    }
}
