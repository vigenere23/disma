use std::sync::Arc;

use crate::{
    category::{CategoriesList, ExistingCategory},
    channel::{ChannelsList, ExistingChannel},
    permission::PermissionsList,
    role::{ExistingRole, RolesList},
};

#[cfg_attr(test, mock_it::mock_it)]
pub trait GuildQuerier {
    // TODO probably add find_<entity>_by_name() -> Result
    // to be used by commands instead of using a whole existing tree
    fn get_guild(&self, guild_id: &str) -> ExistingGuild; // Still needed for computing diffs
    fn list_guilds(&self) -> Vec<GuildSummary>;
}
pub type GuildQuerierRef = Arc<dyn GuildQuerier>;

#[derive(Debug, Clone)]
pub struct ExistingGuild {
    pub roles: RolesList<ExistingRole>,
    pub categories: CategoriesList<ExistingCategory>,
    pub channels: ChannelsList<ExistingChannel>,
}

#[derive(Debug, Clone)]
pub struct GuildSummary {
    pub name: String,
    pub id: String,
    pub nb_members: u128,
    pub permissions: PermissionsList,
}
