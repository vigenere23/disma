use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct MemberResponse {
    pub user: MemberUserResponse,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MemberUserResponse {
    pub username: String,
}
