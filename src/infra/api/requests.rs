use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RoleRequest {
    pub name: String,
    pub permissions: String,
    pub hoist: bool,
    pub mentionable: bool,
}
