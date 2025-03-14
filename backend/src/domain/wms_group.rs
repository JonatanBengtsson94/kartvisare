use super::wms_summary::WmsSummary;

pub struct WmsGroup {
    pub id: i32,
    pub name: String,
    pub sub_groups: Option<Vec<WmsGroup>>,
    pub wms: Option<Vec<WmsSummary>>,
}
