use std::sync::Arc;

use serde::{Deserialize, Serialize};

use disma::{
    category::{
        AwaitingCategoriesList, AwaitingCategory, ExistingCategory, ExtraCategoriesStrategy,
        KeepExtraCategories, RemoveExtraCategories,
    },
    permission::PermissionsOverwrites,
    role::{AwaitingRole, RolesList},
};

use super::{channel::ChannelExtraItemsConfig, permission::PermissionsOverwritesConfig};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CategoryConfigsList {
    #[serde(default = "Vec::default")]
    pub items: Vec<CategoryConfig>,
    #[serde(default = "CategoryExtraItemsConfig::default")]
    pub extra_items: CategoryExtraItemsConfig,
}

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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CategoryExtraItemsConfig {
    pub strategy: CategoryExtraItemsStrategy,
}

impl Default for CategoryExtraItemsConfig {
    fn default() -> Self {
        Self {
            strategy: CategoryExtraItemsStrategy::REMOVE,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CategoryExtraItemsStrategy {
    KEEP,
    REMOVE,
    // TODO Overwrite,
}

impl Into<Arc<dyn ExtraCategoriesStrategy>> for CategoryExtraItemsStrategy {
    fn into(self) -> Arc<dyn ExtraCategoriesStrategy> {
        match self {
            Self::KEEP => Arc::from(KeepExtraCategories {}),
            Self::REMOVE => Arc::from(RemoveExtraCategories {}),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CategoryConfig {
    pub name: String,
    #[serde(default = "Vec::default")]
    pub permissions_overwrites: Vec<PermissionsOverwritesConfig>,
    pub extra_channels: ChannelExtraItemsConfig,
}

impl From<&ExistingCategory> for CategoryConfig {
    fn from(category: &ExistingCategory) -> Self {
        let permissions_overwrites: Vec<PermissionsOverwritesConfig> = category
            .overwrites
            .to_list()
            .iter()
            .map(PermissionsOverwritesConfig::from)
            .collect();

        Self {
            name: category.name.clone(),
            permissions_overwrites,
            extra_channels: ChannelExtraItemsConfig::default(),
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
    mod into_awaiting {
        use std::sync::Arc;

        use disma::{
            category::{
                AwaitingCategoriesList, AwaitingCategory, CategoriesList, KeepExtraCategories,
            },
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

            let matching_entity = AwaitingCategory {
                name: name.to_string(),
                overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrites {
                    role: role.clone(),
                    allow: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                    deny: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                }]),
                extra_channels_strategy: Arc::from(RemoveExtraChannels {}),
            };

            (config, matching_entity)
        }

        fn given_matching_config_list_and_awaiting_entites_list(
            name: &str,
            roles: &RolesList<AwaitingRole>,
        ) -> (CategoryConfigsList, AwaitingCategoriesList) {
            let (config_item, awaiting_item) =
                given_matching_config_and_awaiting_entity(name, roles);

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
            let roles: Vec<AwaitingRole> =
                names.iter().map(|name| given_awaiting_role(name)).collect();
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
            let (config_list, expected_entities_list) =
                given_matching_config_list_and_awaiting_entites_list(name, &roles);

            let entities_list: AwaitingCategoriesList = config_list.into(&roles);

            assert_eq!(entities_list, expected_entities_list);
        }
    }

    mod from_existing {
        use disma::{
            category::ExistingCategory,
            permission::{
                Permission, PermissionsList, PermissionsOverwrites, PermissionsOverwritesList,
            },
            role::ExistingRole,
        };

        use crate::infra::config::{
            category::CategoryConfig,
            channel::{ChannelExtraItemsConfig, ChannelExtraItemsStrategy},
            permission::PermissionsOverwritesConfig,
        };

        fn given_matching_entity_and_config(
            name: &str,
            role: &ExistingRole,
        ) -> (ExistingCategory, CategoryConfig) {
            let entity = ExistingCategory {
                id: "some".to_string(),
                name: name.to_string(),
                overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrites {
                    role: role.clone(),
                    allow: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                    deny: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                }]),
            };

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

            (entity, config)
        }

        fn given_matching_entites_list_and_config_list(name: &str, roles: &ExistingRole) {
            todo!()
        }

        fn given_existing_role(id: &str, name: &str) -> ExistingRole {
            ExistingRole {
                id: id.to_string(),
                name: name.to_string(),
                permissions: PermissionsList::from(vec![]),
                color: None,
                is_mentionable: true,
                show_in_sidebar: false,
            }
        }

        #[test]
        fn can_convert_existing_entity_to_config() {
            let name = "presto";
            let role = given_existing_role("kgj399sd", "Team01");
            let (entity, expected_config) = given_matching_entity_and_config(name, &role);

            let config = CategoryConfig::from(&entity);

            assert_eq!(config, expected_config);
        }

        #[test]
        fn can_convert_existing_entities_list_to_config_list() {
            todo!() // Missing function From<CategoriesList<ExistingCategory>>
        }
    }

    mod serde_parsing {}
}
