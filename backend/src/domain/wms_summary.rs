use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WmsSummary {
    pub id: i32,
    pub name: String,
}
