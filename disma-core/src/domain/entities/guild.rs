use std::sync::Arc;

use crate::channel::{AwaitingChannel, ChannelsList, ExistingChannel};

use super::{
    category::{AwaitingCategory, CategoriesList, ExistingCategory},
    role::{AwaitingRole, ExistingRole, RolesList},
};

pub trait GuildQuerier {
    fn get_guild(&self, guild_id: &str) -> ExistingGuild;
    fn list_guilds(&self) -> Vec<GuildSummary>;
}
pub type GuildQuerierRef = Arc<dyn GuildQuerier>;

pub trait GuildCommander {
    fn add_role(&self, role: &AwaitingRole);
    fn update_role(&self, id: &str, role: &AwaitingRole);
    fn delete_role(&self, id: &str);
    fn add_category(&self, category: &AwaitingCategory, roles: &RolesList<ExistingRole>);
    fn update_category(
        &self,
        id: &str,
        category: &AwaitingCategory,
        roles: &RolesList<ExistingRole>,
    );
    fn delete_category(&self, id: &str);
    fn add_channel(
        &self,
        channel: &AwaitingChannel,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
    );
    fn update_channel(
        &self,
        id: &str,
        channel: &AwaitingChannel,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
    );
    fn delete_channel(&self, id: &str);
}
pub type GuildCommanderRef = Arc<dyn GuildCommander>;

#[derive(Debug)]
pub struct ExistingGuild {
    pub roles: RolesList<ExistingRole>,
    pub categories: CategoriesList<ExistingCategory>,
    pub channels: ChannelsList<ExistingChannel>,
}

#[derive(Debug, PartialEq)]
pub struct AwaitingGuild {
    pub roles: RolesList<AwaitingRole>,
    pub categories: CategoriesList<AwaitingCategory>,
    pub channels: ChannelsList<AwaitingChannel>,
}

pub struct GuildSummary {
    pub name: String,
    pub id: String,
}
