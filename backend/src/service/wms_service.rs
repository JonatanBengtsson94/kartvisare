use crate::{
    domain::{wms_details::WmsDetails, wms_summary::WmsSummary},
    repository::wms_repository::WmsRepository,
};

#[derive(Clone)]
pub struct WmsService<R: WmsRepository> {
    repository: R,
}

impl<R: WmsRepository> WmsService<R> {
    pub fn new(repository: R) -> Self {
        WmsService { repository }
    }

    pub async fn get_wms_summaries(&self) -> Result<Vec<WmsSummary>, sqlx::Error> {
        let wms = self.repository.get_wms_summaries().await?;
        Ok(wms)
    }

    pub async fn get_wms_details(&self, id: i32) -> Result<Option<WmsDetails>, sqlx::Error> {
        let wms = self.repository.get_wms_details(id).await?;
        Ok(wms)
    }
}
