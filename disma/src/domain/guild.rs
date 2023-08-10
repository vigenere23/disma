use std::sync::Arc;

use crate::{
    category::{AwaitingCategoriesList, AwaitingCategory, CategoriesList, ExistingCategory},
    channel::{AwaitingChannel, AwaitingChannelsList, ChannelsList, ExistingChannel},
    role::{AwaitingRole, AwaitingRolesList, ExistingRole, RolesList},
};

#[cfg_attr(test, mock_it::mock_it)]
pub trait GuildQuerier {
    fn get_guild(&self, guild_id: &str) -> ExistingGuild;
    fn list_guilds(&self) -> Vec<GuildSummary>;
}
pub type GuildQuerierRef = Arc<dyn GuildQuerier>;

#[cfg_attr(test, mock_it::mock_it)]
pub trait GuildCommander {
    fn add_role(&self, role: &AwaitingRole) -> Result<(), String>;
    fn update_role(&self, id: &str, role: &AwaitingRole) -> Result<(), String>;
    fn delete_role(&self, id: &str) -> Result<(), String>;
    fn add_category(
        &self,
        category: &AwaitingCategory,
        roles: &RolesList<ExistingRole>,
    ) -> Result<(), String>;
    fn update_category(
        &self,
        id: &str,
        category: &AwaitingCategory,
        roles: &RolesList<ExistingRole>,
    ) -> Result<(), String>;
    fn delete_category(&self, id: &str) -> Result<(), String>;
    fn add_channel(
        &self,
        channel: &AwaitingChannel,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
    ) -> Result<(), String>;
    fn update_channel(
        &self,
        id: &str,
        channel: &AwaitingChannel,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
    ) -> Result<(), String>;
    fn delete_channel(&self, id: &str) -> Result<(), String>;
}
pub type GuildCommanderRef = Arc<dyn GuildCommander>;

#[derive(Debug, Clone)]
pub struct ExistingGuild {
    pub roles: RolesList<ExistingRole>,
    pub categories: CategoriesList<ExistingCategory>,
    pub channels: ChannelsList<ExistingChannel>,
}

#[derive(Debug)]
pub struct AwaitingGuild {
    pub roles: AwaitingRolesList,
    pub categories: AwaitingCategoriesList,
    pub channels: AwaitingChannelsList,
}

#[derive(Debug, Clone)]
pub struct GuildSummary {
    pub name: String,
    pub id: String,
}
