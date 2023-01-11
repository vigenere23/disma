use std::sync::Arc;

use disma::{
    category::{
        AwaitingCategoriesList, AwaitingCategory, ExtraCategoriesStrategy, KeepExtraCategories,
        RemoveExtraCategories,
    },
    permission::PermissionsOverwrites,
    role::{AwaitingRole, RolesList},
};

use super::{CategoryConfig, CategoryConfigsList, CategoryExtraItemsStrategy};

impl CategoryConfigsList {
    pub fn into(self, roles: &RolesList<AwaitingRole>) -> AwaitingCategoriesList {
        let items = self
            .items
            .into_iter()
            .map(|category| category.into(roles))
            .collect::<Vec<AwaitingCategory>>()
            .into();

        AwaitingCategoriesList {
            items,
            extra_items_strategy: self.extra_items.strategy.into(),
        }
    }
}

impl Into<Arc<dyn ExtraCategoriesStrategy>> for CategoryExtraItemsStrategy {
    fn into(self) -> Arc<dyn ExtraCategoriesStrategy> {
        match self {
            Self::KEEP => Arc::from(KeepExtraCategories {}),
            Self::REMOVE => Arc::from(RemoveExtraCategories {}),
        }
    }
}

impl CategoryConfig {
    pub fn into(self, roles: &RolesList<AwaitingRole>) -> AwaitingCategory {
        let overwrites = self
            .permissions_overwrites
            .into_iter()
            .map(|permission| permission.into(roles))
            .collect::<Vec<PermissionsOverwrites<AwaitingRole>>>();

        AwaitingCategory {
            name: self.name,
            overwrites: overwrites.into(),
            extra_channels_strategy: self.extra_channels.strategy.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use disma::{
        category::{AwaitingCategoriesList, AwaitingCategory, CategoriesList, KeepExtraCategories},
        channel::RemoveExtraChannels,
        permission::{
            Permission, PermissionsList, PermissionsOverwrites, PermissionsOverwritesList,
        },
        role::{AwaitingRole, RolesList},
    };

    use crate::infra::config::{
        category::{
            CategoryConfig, CategoryConfigsList, CategoryExtraItemsConfig,
            CategoryExtraItemsStrategy,
        },
        channel::{ChannelExtraItemsConfig, ChannelExtraItemsStrategy},
        permission::PermissionsOverwritesConfig,
    };

    fn given_matching_config_and_awaiting_entity(
        name: &str,
        roles: &RolesList<AwaitingRole>,
    ) -> (CategoryConfig, AwaitingCategory) {
        let role = roles.to_list().first().unwrap();

        let config = CategoryConfig {
            name: name.to_string(),
            permissions_overwrites: vec![PermissionsOverwritesConfig {
                role: role.name.clone(),
                allow: vec![Permission::ADMINISTRATOR],
                deny: vec![Permission::ADMINISTRATOR],
            }],
            extra_channels: ChannelExtraItemsConfig {
                strategy: ChannelExtraItemsStrategy::REMOVE,
            },
        };

        let awaiting_entity = AwaitingCategory {
            name: name.to_string(),
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrites {
                role: role.clone(),
                allow: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                deny: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
            }]),
            extra_channels_strategy: Arc::from(RemoveExtraChannels {}),
        };

        (config, awaiting_entity)
    }

    fn given_matching_config_list_and_awaiting_entites_list(
        name: &str,
        roles: &RolesList<AwaitingRole>,
    ) -> (CategoryConfigsList, AwaitingCategoriesList) {
        let (config_item, awaiting_item) = given_matching_config_and_awaiting_entity(name, roles);

        let config_list = CategoryConfigsList {
            items: vec![config_item],
            extra_items: CategoryExtraItemsConfig {
                strategy: CategoryExtraItemsStrategy::KEEP,
            },
        };

        let awaiting_list = AwaitingCategoriesList {
            items: CategoriesList::from(vec![awaiting_item]),
            extra_items_strategy: Arc::from(KeepExtraCategories {}),
        };

        (config_list, awaiting_list)
    }

    fn given_awaiting_roles(names: Vec<&str>) -> RolesList<AwaitingRole> {
        let roles: Vec<AwaitingRole> = names.iter().map(|name| given_awaiting_role(name)).collect();
        RolesList::from(roles)
    }

    fn given_awaiting_role(name: &str) -> AwaitingRole {
        AwaitingRole {
            name: name.to_string(),
            permissions: PermissionsList::from(vec![]),
            color: None,
            is_mentionable: true,
            show_in_sidebar: false,
        }
    }

    #[test]
    fn can_convert_config_to_awaiting_entity() {
        let name = "presto";
        let roles = given_awaiting_roles(vec!["Team01"]);
        let (config, expected_entity) = given_matching_config_and_awaiting_entity(name, &roles);

        let entity: AwaitingCategory = config.into(&roles);

        assert_eq!(entity, expected_entity);
    }

    #[test]
    fn can_convert_config_list_to_awaiting_entity_list() {
        let name = "presto";
        let roles = given_awaiting_roles(vec!["Team01"]);
        let (config_list, expected_awaiting_list) =
            given_matching_config_list_and_awaiting_entites_list(name, &roles);

        let awaiting_list: AwaitingCategoriesList = config_list.into(&roles);

        assert_eq!(awaiting_list, expected_awaiting_list);
    }
}
