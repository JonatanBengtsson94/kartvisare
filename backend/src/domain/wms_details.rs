use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WmsDetails {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub layers: Vec<String>,
    pub url: String,
    pub version: String,
    pub is_active: bool,
    pub auth_type: String,
    pub auth_username: String,
    pub auth_password: String,
}
