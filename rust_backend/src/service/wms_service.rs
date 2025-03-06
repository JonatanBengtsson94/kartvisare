use crate::repository::wms_repository::WmsRepository;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Client, Error, Response, StatusCode,
};
use std::collections::HashMap;

pub struct WmsService<R: WmsRepository> {
    repository: R,
}

impl<R: WmsRepository> WmsService<R> {
    pub fn new(repository: R) -> Self {
        WmsService { repository }
    }

    pub async fn forward_wms_request(
        &self,
        id: u32,
        query_params: HashMap<String, String>,
    ) -> Result<Response, Error> {
        let wms = match self.repository.get_by_id(id) {
            Some(wms) => wms,
            None => {}
        };
    }
}
