use crate::{
    domain::{
        guild::{ExistingGuild, GuildRepo},
        role::RolesList,
    },
    utils::http::HttpClient,
};

use reqwest::header::{AUTHORIZATION, USER_AGENT};

use super::responses::{ChannelResponse, RoleResponse};

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

    pub fn get_roles(&self) -> Vec<RoleResponse> {
        let url = format!("/guilds/{}/roles", &self.guild_id);
        self.client.get(&url)
    }

    pub fn get_channels(&self) -> Vec<ChannelResponse> {
        let url = format!("/guilds/{}/channels", &self.guild_id);
        self.client.get(&url)
    }
}

// TODO

impl GuildRepo for DiscordApi {
    fn guild(&self) -> ExistingGuild {
        let roles = self.get_roles();
        ExistingGuild {
            roles: RolesList::new(roles.into_iter().map(|value| value.into()).collect()),
        }
    }
}
// impl AwaitingGuild for DiscordApi {}
