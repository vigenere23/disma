pub mod requests;
pub mod responses;

use crate::{
    domain::{
        guild::{ExistingGuild, GuildCommander, GuildQuerier},
        role::{AwaitingRole, ExistingRolesList},
    },
    utils::http::HttpClient,
};

use reqwest::header::{AUTHORIZATION, USER_AGENT};

use self::{
    requests::RoleRequest,
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
        Self { client, guild_id }
    }

    fn get_roles(&self) -> Vec<RoleResponse> {
        let url = format!("/guilds/{}/roles", &self.guild_id);
        self.client.get(&url)
    }

    pub fn _get_channels(&self) -> Vec<ChannelResponse> {
        let url = format!("/guilds/{}/channels", &self.guild_id);
        self.client.get(&url)
    }
}

impl GuildQuerier for DiscordApi {
    fn guild(&self) -> ExistingGuild {
        let roles = self.get_roles();
        ExistingGuild {
            roles: ExistingRolesList::new(roles.into_iter().map(|value| value.into()).collect()),
        }
    }
}

impl GuildCommander for DiscordApi {
    fn add_role(&self, role: &AwaitingRole) {
        let url = format!("/guilds/{}/roles", &self.guild_id);
        self.client.post(&url, Some(RoleRequest::from(role)))
    }

    fn update_role(&self, id: &str, role: &AwaitingRole) {
        let url = format!("/guilds/{}/roles/{}", &self.guild_id, id);
        self.client.patch(&url, Some(RoleRequest::from(role)));
    }

    fn delete_role(&self, id: &str) {
        let url = format!("/guilds/{}/roles/{}", &self.guild_id, id);
        self.client.delete(&url);
    }
}
