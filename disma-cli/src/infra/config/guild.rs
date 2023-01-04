use serde::{Deserialize, Serialize};

use disma::{
    guild::{AwaitingGuild, ExistingGuild},
    utils::vec::Compress,
};

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
                items: roles.compress(),
                extra_items: RoleExtraItemsConfig::default(),
            },
            categories: CategoryConfigsList {
                items: categories.compress(),
                extra_items: CategoryExtraItemsConfig::default(),
            },
            channels: ChannelConfigsList {
                items: channels.compress(),
                extra_items: ChannelExtraItemsConfig::default(),
            },
        }
    }
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

#[cfg(test)]
mod tests {
    use disma::{
        category::{AwaitingCategoriesList, CategoriesList},
        channel::{AwaitingChannelsList, ChannelsList},
        guild::{AwaitingGuild, ExistingGuild},
        role::{AwaitingRolesList, RolesList},
    };

    use crate::infra::config::{
        category::{CategoryConfigsList, CategoryExtraItemsConfig},
        channel::{ChannelConfigsList, ChannelExtraItemsConfig},
        role::{RoleConfigsList, RoleExtraItemsConfig},
    };

    use super::GuildConfig;

    #[test]
    pub fn when_converting_to_awaiting_guild_then_nones_are_converted_to_defaults() {
        let config = GuildConfig {
            roles: RoleConfigsList::default(),
            categories: CategoryConfigsList::default(),
            channels: ChannelConfigsList::default(),
        };

        let entity: AwaitingGuild = config.into();

        let expected_entity = AwaitingGuild {
            roles: AwaitingRolesList {
                items: RolesList::from(vec![]),
                extra_items_strategy: RoleExtraItemsConfig::default().strategy.into(),
            },
            categories: AwaitingCategoriesList {
                items: CategoriesList::from(vec![]),
                extra_items_strategy: CategoryExtraItemsConfig::default().strategy.into(),
            },
            channels: AwaitingChannelsList {
                items: ChannelsList::from(vec![]),
                extra_items_strategy: ChannelExtraItemsConfig::default().strategy.into(),
            },
        };
        assert_eq!(entity, expected_entity);
    }

    #[test]
    pub fn when_parsing_existing_guild_then_empty_arrays_are_converted_to_nones_and_nones_to_defaults(
    ) {
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
