pub mod requests;
pub mod responses;

use crate::{
    domain::{
        guild::{AwaitingGuild, ExistingGuild, GuildRepo},
        role::RolesList,
    },
    utils::http::HttpClient,
};

use reqwest::header::{AUTHORIZATION, USER_AGENT};

use self::responses::{ChannelResponse, RoleResponse};

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

    pub fn delete_role(&self, id: &str) {
        let url = format!("/guilds/{}/roles/{}", &self.guild_id, id);
        self.client.delete(&url);
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

impl AwaitingGuild for DiscordApi {
    fn add_role(&self, role: &crate::domain::role::AwaitingRole) {
        todo!()
    }

    fn delete_role(&self, id: &str) {
        todo!()
    }
}
