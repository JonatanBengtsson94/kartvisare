#[derive(Clone, Debug)]
pub struct Session {
    pub session_id: String,
    pub user_id: i32,
    pub is_admin: bool,
}
