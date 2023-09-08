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
    roles: RolesList<ExistingRole>,
    categories: CategoriesList<ExistingCategory>,
    channels: ChannelsList<ExistingChannel>,
}

impl ExistingGuild {
    pub fn new(
        roles: RolesList<ExistingRole>,
        categories: CategoriesList<ExistingCategory>,
        channels: ChannelsList<ExistingChannel>,
    ) -> Self {
        Self {
            roles,
            categories,
            channels,
        }
    }

    pub fn roles(&self) -> &RolesList<ExistingRole> {
        &self.roles
    }

    pub fn add_or_replace_role(&mut self, role: ExistingRole) {
        self.roles.add_or_replace(role)
    }

    pub fn remove_role(&mut self, _role: ExistingRole) {
        todo!()
    }

    pub fn categories(&self) -> &CategoriesList<ExistingCategory> {
        &self.categories
    }

    pub fn add_or_replace_category(&mut self, category: ExistingCategory) {
        // TODO check for existing roles??
        // Or reput complex objects instead of references?

        self.categories.add_or_replace(category)
    }

    pub fn channels(&self) -> &ChannelsList<ExistingChannel> {
        &self.channels
    }

    pub fn add_or_replace_channel(&mut self, channel: ExistingChannel) {
        self.channels.add_or_replace(channel)
    }
}

#[derive(Debug, Clone)]
pub struct GuildSummary {
    pub name: String,
    pub id: String,
    pub nb_members: u128,
    pub permissions: PermissionsList,
}
