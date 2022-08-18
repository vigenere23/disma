pub mod requests;
pub mod responses;

use crate::{
    domain::{
        guild::{ExistingGuild, GuildCommander, GuildQuerier},
        role::{AwaitingRole, ExistingRolesList},
    },
    utils::http::{Client, ClientBuilder},
};

use reqwest::header::{AUTHORIZATION, USER_AGENT};

use self::{
    requests::RoleRequest,
    responses::{ChannelResponse, RoleResponse},
};

pub struct DiscordApi {
    client: Client,
    guild_id: String,
}

impl DiscordApi {
    pub fn from_bot(bot_token: String, guild_id: String) -> DiscordApi {
        let client = ClientBuilder::new()
            .base_url("https://discord.com/api/v9")
            .header(USER_AGENT, "")
            .header(AUTHORIZATION, &format!("Bot {}", bot_token))
            .build();
        Self { client, guild_id }
    }

    fn get_roles(&self) -> Vec<RoleResponse> {
        let url = format!("/guilds/{}/roles", &self.guild_id);
        self.client.get(&url).send().unwrap().parsed_body().unwrap()
    }

    pub fn _get_channels(&self) -> Vec<ChannelResponse> {
        let url = format!("/guilds/{}/channels", &self.guild_id);
        self.client.get(&url).send().unwrap().parsed_body().unwrap()
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
        self.client
            .post(&url)
            .json_body(RoleRequest::from(role))
            .unwrap()
            .send()
            .unwrap();
    }

    fn update_role(&self, id: &str, role: &AwaitingRole) {
        let url = format!("/guilds/{}/roles/{}", &self.guild_id, id);
        self.client
            .patch(&url)
            .json_body(RoleRequest::from(role))
            .unwrap()
            .send()
            .unwrap();
    }

    fn delete_role(&self, id: &str) {
        let url = format!("/guilds/{}/roles/{}", &self.guild_id, id);
        self.client.delete(&url).send().unwrap();
    }
}
