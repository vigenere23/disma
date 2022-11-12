use serde::{Deserialize, Serialize};

use disma::{
    category::{AwaitingCategory, CategoriesList},
    guild::{AwaitingGuild, ExistingGuild},
    role::{AwaitingRole, RolesList},
    utils::vec::Compress,
};

use super::{category::CategoryConfig, channel::ChannelConfig, role::RoleConfig};

#[derive(Serialize, Deserialize)]
pub struct GuildConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<RoleConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<Vec<CategoryConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channels: Option<Vec<ChannelConfig>>,
}

impl From<&ExistingGuild> for GuildConfig {
    fn from(guild: &ExistingGuild) -> Self {
        let roles: Vec<RoleConfig> = guild.roles.items().iter().map(|role| role.into()).collect();

        let categories: Vec<CategoryConfig> = guild
            .categories
            .items()
            .iter()
            .map(CategoryConfig::from)
            .collect();

        let channels: Vec<ChannelConfig> = guild
            .channels
            .iter()
            .map(|channel| channel.into())
            .collect();

        Self {
            roles: roles.compress(),
            categories: categories.compress(),
            channels: channels.compress(),
        }
    }
}

impl Into<AwaitingGuild> for GuildConfig {
    fn into(self) -> AwaitingGuild {
        let roles: Vec<AwaitingRole> = self
            .roles
            .unwrap_or_default()
            .into_iter()
            .map(|role_config| role_config.into())
            .collect();

        let roles_list = RolesList::from(roles);

        let categories: Vec<AwaitingCategory> = self
            .categories
            .unwrap_or_default()
            .into_iter()
            .map(|category| category.into(&roles_list))
            .collect();

        AwaitingGuild {
            roles: roles_list,
            categories: CategoriesList::from(categories),
        }
    }
}
