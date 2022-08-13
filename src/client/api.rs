use crate::utils::http::HttpClient;

use async_trait::async_trait;
use reqwest::header::{AUTHORIZATION, USER_AGENT};

use super::{
    base::DiscordClient,
    responses::{ChannelResponse, RoleResponse},
};

pub struct DiscordApi {
    client: HttpClient,
    guild_id: String,
}

impl DiscordApi {
    pub fn new(token: String, guild_id: String) -> DiscordApi {
        let client = HttpClient::builder()
            .base_url("https://discord.com/api/v9")
            .header(USER_AGENT, String::new())
            .header(AUTHORIZATION, format!("Bot {}", token))
            .build();
        DiscordApi { client, guild_id }
    }
}

#[async_trait]
impl DiscordClient for DiscordApi {
    async fn get_roles(&self) -> Vec<RoleResponse> {
        let url = format!("/guilds/{}/roles", &self.guild_id);
        self.client.get(&url).await
    }

    async fn get_channels(&self) -> Vec<ChannelResponse> {
        let url = format!("/guilds/{}/channels", &self.guild_id);
        self.client.get(&url).await
    }
}
