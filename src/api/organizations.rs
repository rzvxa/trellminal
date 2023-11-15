use super::{Api, ENDPOINT, Request};
use crate::models::Organization;
use const_format::formatcp;

const URL_BASE: &str = formatcp!("{}/organizations", ENDPOINT);

pub trait Organizations {
    fn organizations_get(
        &self,
        org_id: String,
    ) -> Request<Organization>;
}

impl Organizations for Api {
    fn organizations_get(
        &self,
        org_id: String,
    ) -> Request<Organization> {
        let fetch_user_url = format!(
            "{}/{}?key={}&token={}",
            URL_BASE, org_id, self.key, self.token
        );
        self.get_req(fetch_user_url)
    }
}