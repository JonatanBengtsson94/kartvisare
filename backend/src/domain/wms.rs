pub struct Wms {
    id: i32,
    name: String,
    description: String,
    layers: Vec<String>,
    url: String,
    version: String,
    is_active: bool,
    auth_type: String,
    auth_username: String,
    auth_password: String,
}
