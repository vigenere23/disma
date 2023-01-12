use std::sync::Arc;

use disma::{
    category::{AwaitingCategory, CategoriesList},
    channel::{
        AwaitingChannel, AwaitingChannelsList, ChannelType, ExtraChannelsStrategy,
        KeepExtraChannels, RemoveExtraChannels,
    },
    permission::PermissionsOverwrites,
    role::{AwaitingRole, RolesList},
};

use super::{ChannelConfig, ChannelConfigType, ChannelConfigsList, ChannelExtraItemsStrategy};

impl ChannelConfigsList {
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
        }
    }
}

impl Into<Arc<dyn ExtraChannelsStrategy>> for ChannelExtraItemsStrategy {
    fn into(self) -> Arc<dyn ExtraChannelsStrategy> {
        match self {
            Self::KEEP => Arc::from(KeepExtraChannels {}),
            Self::REMOVE => Arc::from(RemoveExtraChannels {}),
        }
    }
}

impl ChannelConfig {
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

        // TODO add strategy to inherit category's overwrites?
        let overwrites = self
            .permissions_overwrites
            .into_iter()
            .map(|permission| permission.into(roles))
            .collect::<Vec<PermissionsOverwrites<AwaitingRole>>>();

        AwaitingChannel {
            name: self.name,
            topic: self.topic,
            channel_type,
            category,
            overwrites: overwrites.into(),
        }
    }
}

impl Into<ChannelType> for ChannelConfigType {
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

    use disma::{
        category::{AwaitingCategory, CategoriesList},
        channel::{
            AwaitingChannel, AwaitingChannelsList, ChannelType, ChannelsList, KeepExtraChannels,
        },
        permission::{
            Permission, PermissionsList, PermissionsOverwrites, PermissionsOverwritesList,
        },
        role::{AwaitingRole, RolesList},
    };

    use crate::infra::config::{
        channel::{
            ChannelConfig, ChannelConfigType, ChannelConfigsList, ChannelExtraItemsConfig,
            ChannelExtraItemsStrategy,
        },
        permission::PermissionsOverwritesConfig,
    };

    fn given_awaiting_category(name: &str) -> AwaitingCategory {
        AwaitingCategory {
            name: name.to_string(),
            overwrites: PermissionsOverwritesList::from(vec![]),
            extra_channels_strategy: ChannelExtraItemsConfig::default().strategy.into(),
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

    fn given_matching_config_and_awaiting(
        name: &str,
        roles: &RolesList<AwaitingRole>,
        categories: &CategoriesList<AwaitingCategory>,
    ) -> (ChannelConfig, AwaitingChannel) {
        let role = roles.to_list().first().unwrap();
        let category = categories.to_list().first().unwrap();

        let config = ChannelConfig {
            name: name.to_string(),
            _type: ChannelConfigType::VOICE,
            category: Some(category.name.clone()),
            topic: Some("Nice sweater".to_string()),
            permissions_overwrites: vec![PermissionsOverwritesConfig {
                role: role.name.clone(),
                allow: vec![Permission::ADMINISTRATOR],
                deny: vec![Permission::SEND_MESSAGES],
            }],
        };

        let awaiting = AwaitingChannel {
            name: name.to_string(),
            channel_type: ChannelType::VOICE,
            category: Some(category.clone()),
            topic: Some("Nice sweater".to_string()),
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrites {
                role: role.clone(),
                allow: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                deny: PermissionsList::from(vec![Permission::SEND_MESSAGES]),
            }]),
        };

        (config, awaiting)
    }

    fn given_matching_config_list_and_awaiting_list(
        name: &str,
        roles: &RolesList<AwaitingRole>,
        categories: &CategoriesList<AwaitingCategory>,
    ) -> (ChannelConfigsList, AwaitingChannelsList) {
        let (config_item, awaiting_item) =
            given_matching_config_and_awaiting(name, roles, categories);

        let config_list = ChannelConfigsList {
            items: vec![config_item],
            extra_items: ChannelExtraItemsConfig {
                strategy: ChannelExtraItemsStrategy::KEEP,
            },
        };

        let awaiting_list = AwaitingChannelsList {
            items: ChannelsList::from(vec![awaiting_item]),
            extra_items_strategy: Arc::from(KeepExtraChannels {}),
        };

        (config_list, awaiting_list)
    }

    #[test]
    fn can_convert_config_to_awaiting_entity() {
        let name = "channel_1";
        let categories = given_awaiting_categories(vec!["category_1"]);
        let roles = given_awaiting_roles(vec!["role_1"]);
        let (config, expected_awaiting) =
            given_matching_config_and_awaiting(name, &roles, &categories);

        let awaiting = config.into(&roles, &categories);

        assert_eq!(awaiting, expected_awaiting);
    }

    #[test]
    fn can_convert_config_list_to_awaiting_entity_list() {
        let name = "channel_1";
        let categories = given_awaiting_categories(vec!["category_1"]);
        let roles = given_awaiting_roles(vec!["role_1"]);
        let (config_list, expected_awaiting_list) =
            given_matching_config_list_and_awaiting_list(name, &roles, &categories);

        let awaiting_list: AwaitingChannelsList = config_list.into(&roles, &categories);

        assert_eq!(awaiting_list, expected_awaiting_list);
    }
}
