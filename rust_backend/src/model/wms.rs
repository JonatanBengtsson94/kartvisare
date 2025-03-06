use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Wms {
    pub id: u32,
    pub name: String,
    pub url: String,
    pub version: Option<String>,
    pub is_active: bool,
    pub auth_type: Option<String>,
    pub auth_username: Option<String>,
    pub auth_password: Option<String>,
}
