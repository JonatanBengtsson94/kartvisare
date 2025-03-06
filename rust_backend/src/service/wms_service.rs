use crate::repository::wms_repository::WmsRepository;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Client, StatusCode,
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
    ) -> Result<reqwest::Response, axum::Error> {
        let wms = match self.repository.get_by_id(id) {
            Some(wms) => wms,
            None => {
                return Err(axum::Error::new("WMS not found").into());
            }
        };

        let mut request_builder = Client::new().get(&wms.url);

        // Handle wms authentication here

        let response = request_builder
            .query(&query_params)
            .send()
            .await
            .map_err(|e| axum::Error::new(format!("Failed to forward WMS request: {}", e)))?;

        Ok(response)
    }
}
