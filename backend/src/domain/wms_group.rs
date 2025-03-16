use super::wms_details::WmsDetails;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WmsGroup {
    pub id: i32,
    pub name: String,
    pub sub_groups: Option<Vec<WmsGroup>>,
    pub wms: Option<Vec<WmsDetails>>,
}
