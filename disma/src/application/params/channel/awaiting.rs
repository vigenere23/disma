use std::sync::Arc;

use crate::{
    category::{AwaitingCategory, CategoriesList},
    channel::{
        AwaitingChannel, AwaitingChannelsList, ChannelType, ExtraChannelsStrategy,
        KeepExtraChannels, RemoveExtraChannels,
    },
    permission::PermissionsOverwrite,
    role::{AwaitingRole, RolesList},
};

use super::{
    ChannelParams, ChannelParamsChannelType, ChannelParamsExtraItemsStrategy, ChannelsParamsList,
};

impl ChannelsParamsList {
    pub fn into(
        self,
        roles: &RolesList<AwaitingRole>,
        categories: &CategoriesList<AwaitingCategory>,
    ) -> AwaitingChannelsList {
        let items = self
            .items
            .into_iter()
            .map(|channel| channel.into(roles, categories))
            .collect::<Vec<AwaitingChannel>>()
            .into();

        AwaitingChannelsList {
            items,
            extra_items_strategy: self.extra_items.strategy.into(),
            categories: categories.clone(),
        }
    }
}

impl Into<Arc<dyn ExtraChannelsStrategy>> for ChannelParamsExtraItemsStrategy {
    fn into(self) -> Arc<dyn ExtraChannelsStrategy> {
        match self {
            Self::KEEP => Arc::from(KeepExtraChannels {}),
            Self::REMOVE => Arc::from(RemoveExtraChannels {}),
        }
    }
}

impl ChannelParams {
    pub fn into(
        self,
        roles: &RolesList<AwaitingRole>,
        categories: &CategoriesList<AwaitingCategory>,
    ) -> AwaitingChannel {
        let channel_type = self._type.into();

        let category = self.category.map(|name| {
            categories
                .find_by_name(&name)
                .unwrap_or_else(|| panic!("No category found for name {name}."))
                .clone()
        });

        let overwrites = self.permissions_overwrites.map(|permissions| {
            permissions
                .into_iter()
                .map(|permission| permission.into(roles))
                .collect::<Vec<PermissionsOverwrite<AwaitingRole>>>()
                .into()
        });

        AwaitingChannel {
            name: self.name,
            topic: self.topic,
            channel_type,
            category,
            overwrites,
        }
    }
}

impl Into<ChannelType> for ChannelParamsChannelType {
    fn into(self) -> ChannelType {
        match self {
            Self::TEXT => ChannelType::TEXT,
            Self::VOICE => ChannelType::VOICE,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        category::{AwaitingCategory, CategoriesList},
        channel::{
            AwaitingChannel, AwaitingChannelsList, ChannelType, ChannelsList, KeepExtraChannels,
        },
        params::{
            channel::{
                ChannelParams, ChannelParamsChannelType, ChannelParamsExtraItems,
                ChannelParamsExtraItemsStrategy, ChannelsParamsList,
            },
            permission::PermissionsOverwriteParams,
        },
        permission::{
            Permission, PermissionsList, PermissionsOverwrite, PermissionsOverwritesList,
        },
        role::{AwaitingRole, RolesList},
    };

    fn given_awaiting_category(name: &str) -> AwaitingCategory {
        AwaitingCategory {
            name: name.to_string(),
            overwrites: PermissionsOverwritesList::from(vec![]),
            sync_permissions: false,
            extra_channels_strategy: ChannelParamsExtraItems::default().strategy.into(),
        }
    }

    fn given_awaiting_categories(names: Vec<&str>) -> CategoriesList<AwaitingCategory> {
        let categories: Vec<AwaitingCategory> = names
            .iter()
            .map(|name| given_awaiting_category(name))
            .collect();
        CategoriesList::from(categories)
    }

    fn given_awaiting_role(name: &str) -> AwaitingRole {
        AwaitingRole {
            name: name.to_string(),
            permissions: PermissionsList::from(vec![Permission::VIEW_CHANNEL]),
            color: Some("123456".to_string()),
            is_mentionable: true,
            show_in_sidebar: false,
        }
    }

    fn given_awaiting_roles(names: Vec<&str>) -> RolesList<AwaitingRole> {
        let roles: Vec<AwaitingRole> = names.iter().map(|name| given_awaiting_role(name)).collect();
        RolesList::from(roles)
    }

    fn given_matching_params_and_awaiting(
        name: &str,
        roles: &RolesList<AwaitingRole>,
        categories: &CategoriesList<AwaitingCategory>,
    ) -> (ChannelParams, AwaitingChannel) {
        let role = roles.to_list().first().unwrap();
        let category = categories.to_list().first().unwrap();

        let params = ChannelParams {
            name: name.to_string(),
            _type: ChannelParamsChannelType::VOICE,
            category: Some(category.name.clone()),
            topic: Some("Nice sweater".to_string()),
            permissions_overwrites: Some(vec![PermissionsOverwriteParams {
                role: role.name.clone(),
                allow: vec![Permission::ADMINISTRATOR],
                deny: vec![Permission::SEND_MESSAGES],
            }]),
        };

        let awaiting = AwaitingChannel {
            name: name.to_string(),
            channel_type: ChannelType::VOICE,
            category: Some(category.clone()),
            topic: Some("Nice sweater".to_string()),
            overwrites: Some(PermissionsOverwritesList::from(vec![
                PermissionsOverwrite {
                    role: role.clone(),
                    allow: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                    deny: PermissionsList::from(vec![Permission::SEND_MESSAGES]),
                },
            ])),
        };

        (params, awaiting)
    }

    fn given_matching_params_list_and_awaiting_list(
        name: &str,
        roles: &RolesList<AwaitingRole>,
        categories: &CategoriesList<AwaitingCategory>,
    ) -> (ChannelsParamsList, AwaitingChannelsList) {
        let (params, awaiting) = given_matching_params_and_awaiting(name, roles, categories);

        let params_list = ChannelsParamsList {
            items: vec![params],
            extra_items: ChannelParamsExtraItems {
                strategy: ChannelParamsExtraItemsStrategy::KEEP,
            },
        };

        let awaiting_list = AwaitingChannelsList {
            items: ChannelsList::from(vec![awaiting]),
            extra_items_strategy: Arc::from(KeepExtraChannels {}),
            categories: categories.clone(),
        };

        (params_list, awaiting_list)
    }

    #[test]
    fn can_convert_params_to_awaiting_entity() {
        let name = "channel_1";
        let categories = given_awaiting_categories(vec!["category_1"]);
        let roles = given_awaiting_roles(vec!["role_1"]);
        let (params, expected_awaiting) =
            given_matching_params_and_awaiting(name, &roles, &categories);

        let awaiting = params.into(&roles, &categories);

        assert_eq!(awaiting, expected_awaiting);
    }

    #[test]
    fn can_convert_params_list_to_awaiting_entity_list() {
        let name = "channel_1";
        let categories = given_awaiting_categories(vec!["category_1"]);
        let roles = given_awaiting_roles(vec!["role_1"]);
        let (params_list, expected_awaiting_list) =
            given_matching_params_list_and_awaiting_list(name, &roles, &categories);

        let awaiting_list: AwaitingChannelsList = params_list.into(&roles, &categories);

        assert_eq!(awaiting_list, expected_awaiting_list);
    }
}
