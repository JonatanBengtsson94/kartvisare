use super::wms_summary::WmsSummary;

pub struct WmsGroup {
    id: i32,
    name: String,
    sub_groups: Vec<WmsGroup>,
    wms: Vec<WmsSummary>,
}
