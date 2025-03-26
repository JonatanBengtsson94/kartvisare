use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub user_name: String,
    pub idp_id: String,
}
