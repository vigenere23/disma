use serde::{Deserialize, Serialize};

use disma::{
    category::{AwaitingCategory, CategoriesList},
    channel::{AwaitingChannel, ChannelsList},
    guild::{AwaitingGuild, ExistingGuild},
    role::{AwaitingRole, RolesList},
    utils::vec::Compress,
};

use super::{
    category::CategoryConfig, channel::ChannelConfig, options::OptionsConfig, role::RoleConfig,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GuildConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<RoleConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<Vec<CategoryConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channels: Option<Vec<ChannelConfig>>,
    #[serde(default = "OptionsConfig::default")]
    options: OptionsConfig,
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
            .items()
            .iter()
            .map(|channel| channel.into())
            .collect();

        Self {
            roles: roles.compress(),
            categories: categories.compress(),
            channels: channels.compress(),
            options: OptionsConfig::default(),
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

        let options = self.options.into();

        AwaitingGuild {
            roles: roles_list,
            categories: categories_list,
            channels: channels_list,
            options,
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
            options: OptionsConfig::default(),
        };

        let entity: AwaitingGuild = config.into();

        let expected_entity = AwaitingGuild {
            roles: RolesList::from(vec![]),
            categories: CategoriesList::from(vec![]),
            channels: ChannelsList::from(vec![]),
            options: AwaitingGuildOptions {
                roles: AwaitingRolesOptions { allow_extra: false },
                categories: AwaitingCategoriesOptions { allow_extra: false },
                channels: AwaitingChannelsOptions { allow_extra: false },
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
            roles: None,
            categories: None,
            channels: None,
            options: OptionsConfig::default(),
        };
        assert_eq!(config, expected_config);
    }
}
