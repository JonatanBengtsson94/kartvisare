use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WmsDetails {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub layers: Vec<String>,
    pub url: String,
    pub version: Option<String>,
    pub is_active: bool,
    pub auth_type: Option<String>,
    pub auth_username: Option<String>,
    pub auth_password: Option<String>,
}
