use super::responses::{ChannelResponse, RoleResponse};
use async_trait::async_trait;

#[async_trait]
pub trait DiscordClient {
    async fn get_roles(&self) -> Vec<RoleResponse>;
    async fn get_channels(&self) -> Vec<ChannelResponse>;
}
