use serde::{Deserialize, Serialize};

use crate::guild::{AwaitingGuild, ExistingGuild};

use super::{
    category::{CategoriesParamsList, CategoryParams, CategoryParamsExtraItemsStrategy},
    channel::{ChannelParams, ChannelParamsExtraItemsStrategy, ChannelsParamsList},
    role::{RoleParams, RoleParamsExtraItemsStrategy, RolesParamsList},
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GuildParams {
    #[serde(default = "RolesParamsList::default")]
    pub roles: RolesParamsList,
    #[serde(default = "CategoriesParamsList::default")]
    pub categories: CategoriesParamsList,
    #[serde(default = "ChannelsParamsList::default")]
    pub channels: ChannelsParamsList,
}

impl Into<AwaitingGuild> for GuildParams {
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

impl From<&ExistingGuild> for GuildParams {
    fn from(guild: &ExistingGuild) -> Self {
        let roles: Vec<RoleParams> = guild
            .roles()
            .to_list()
            .into_iter()
            .map(RoleParams::from)
            .collect();

        let categories: Vec<CategoryParams> = guild
            .categories
            .to_list()
            .into_iter()
            .map(CategoryParams::from)
            .collect();

        let channels: Vec<ChannelParams> = guild
            .channels
            .to_list()
            .into_iter()
            .map(ChannelParams::from)
            .collect();

        Self {
            roles: RolesParamsList {
                items: roles,
                extra_items: RoleParamsExtraItemsStrategy::default(),
            },
            categories: CategoriesParamsList {
                items: categories,
                extra_items: CategoryParamsExtraItemsStrategy::default(),
            },
            channels: ChannelsParamsList {
                items: channels,
                extra_items: ChannelParamsExtraItemsStrategy::default(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        category::CategoriesList, channel::ChannelsList, guild::ExistingGuild, role::RolesList,
    };

    use crate::api::params::{
        category::CategoriesParamsList, channel::ChannelsParamsList, role::RolesParamsList,
    };

    use super::GuildParams;

    #[test]
    pub fn when_parsing_empty_existing_guild_it_fills_params_with_defaults() {
        let entity =
            ExistingGuild::new(RolesList::new(), CategoriesList::new(), ChannelsList::new());

        let params = GuildParams::from(&entity);

        let expected_params = GuildParams {
            roles: RolesParamsList::default(),
            categories: CategoriesParamsList::default(),
            channels: ChannelsParamsList::default(),
        };
        assert_eq!(params, expected_params);
    }
}
