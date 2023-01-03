use serde::{Deserialize, Serialize};

use disma::{
    category::{AwaitingCategory, CategoriesList},
    channel::{AwaitingChannel, ChannelsList},
    guild::{AwaitingGuild, ExistingGuild},
    role::{AwaitingRole, RolesList},
    utils::vec::Compress,
};

use super::{
    category::{CategoryConfig, CategoryConfigsList},
    channel::{ChannelConfig, ChannelConfigsList},
    role::{RoleConfig, RoleConfigsList},
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
        let roles: Vec<RoleConfig> = guild
            .roles
            .to_list()
            .iter()
            .map(|role| role.into())
            .collect();

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

        let categories_list = CategoriesList::from(categories);

        let channels: Vec<AwaitingChannel> = self
            .channels
            .unwrap_or_default()
            .into_iter()
            .map(|channel| channel.into(&roles_list, &categories_list))
            .collect();

        let channels_list = ChannelsList::from(channels);

        AwaitingGuild {
            roles: roles_list,
            categories: categories_list,
            channels: channels_list,
        }
    }
}

#[cfg(test)]
mod tests {
    use disma::{
        category::CategoriesList,
        channel::ChannelsList,
        guild::{
            AwaitingCategoriesOptions, AwaitingChannelsOptions, AwaitingGuild,
            AwaitingGuildOptions, AwaitingRolesOptions, ExistingGuild,
        },
        role::RolesList,
    };

    use crate::infra::config::options::OptionsConfig;

    use super::GuildConfig;

    #[test]
    pub fn when_converting_to_awaiting_guild_then_nones_are_converted_to_defaults() {
        let config = GuildConfig {
            roles: None,
            categories: None,
            channels: None,
        };

        let entity: AwaitingGuild = config.into();

        let expected_entity = AwaitingGuild {
            roles: RolesList::from(vec![]),
            categories: CategoriesList::from(vec![]),
            channels: ChannelsList::from(vec![]),
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
            roles: None,
            categories: None,
            channels: None,
        };
        assert_eq!(config, expected_config);
    }
}
