use serde::{Deserialize, Serialize};

use disma::guild::{AwaitingGuild, ExistingGuild};

use super::{
    category::{CategoryConfig, CategoryConfigsList, CategoryExtraItemsConfig},
    channel::{ChannelConfig, ChannelConfigsList, ChannelExtraItemsConfig},
    role::{RoleConfig, RoleConfigsList, RoleExtraItemsConfig},
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GuildConfig {
    #[serde(default = "RoleConfigsList::default")]
    roles: RoleConfigsList,
    #[serde(default = "CategoryConfigsList::default")]
    categories: CategoryConfigsList,
    #[serde(default = "ChannelConfigsList::default")]
    channels: ChannelConfigsList,
}

impl Into<AwaitingGuild> for GuildConfig {
    fn into(self) -> AwaitingGuild {
        let roles = self.roles.into();
        let categories = self.categories.into(&roles.items);
        let channels = self.channels.into(&roles.items, &categories.items);

        AwaitingGuild {
            roles,
            categories,
            channels,
        }
    }
}

impl From<&ExistingGuild> for GuildConfig {
    fn from(guild: &ExistingGuild) -> Self {
        let roles: Vec<RoleConfig> = guild.roles.to_list().iter().map(RoleConfig::from).collect();

        let categories: Vec<CategoryConfig> = guild
            .categories
            .to_list()
            .iter()
            .map(CategoryConfig::from)
            .collect();

        let channels: Vec<ChannelConfig> = guild
            .channels
            .to_list()
            .iter()
            .map(|channel| channel.into())
            .collect();

        Self {
            roles: RoleConfigsList {
                items: roles,
                extra_items: RoleExtraItemsConfig::default(),
            },
            categories: CategoryConfigsList {
                items: categories,
                extra_items: CategoryExtraItemsConfig::default(),
            },
            channels: ChannelConfigsList {
                items: channels,
                extra_items: ChannelExtraItemsConfig::default(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use disma::{
        category::CategoriesList, channel::ChannelsList, guild::ExistingGuild, role::RolesList,
    };

    use crate::infra::config::{
        category::CategoryConfigsList, channel::ChannelConfigsList, role::RoleConfigsList,
    };

    use super::GuildConfig;

    #[test]
    pub fn when_parsing_empty_existing_guild_it_fills_config_with_defaults() {
        let entity = ExistingGuild {
            roles: RolesList::from(vec![]),
            categories: CategoriesList::from(vec![]),
            channels: ChannelsList::from(vec![]),
        };

        let config = GuildConfig::from(&entity);

        let expected_config = GuildConfig {
            roles: RoleConfigsList::default(),
            categories: CategoryConfigsList::default(),
            channels: ChannelConfigsList::default(),
        };
        assert_eq!(config, expected_config);
    }
}
