use std::sync::Arc;

use serde::{Deserialize, Serialize};

use disma::{
    category::{
        AwaitingCategoriesList, AwaitingCategory, ExistingCategory, ExtraCategoriesStrategy,
        KeepExtraCategories, RemoveExtraCategories,
    },
    permission::PermissionsOverwrites,
    role::{AwaitingRole, RolesList},
    utils::vec::Compress,
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
            strategy: CategoryExtraItemsStrategy::Remove,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CategoryExtraItemsStrategy {
    Keep,
    Remove,
    // TODO Overwrite,
}

impl Into<Arc<dyn ExtraCategoriesStrategy>> for CategoryExtraItemsStrategy {
    fn into(self) -> Arc<dyn ExtraCategoriesStrategy> {
        match self {
            Self::Keep => Arc::from(KeepExtraCategories {}),
            Self::Remove => Arc::from(RemoveExtraCategories {}),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CategoryConfig {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_overwrites: Option<Vec<PermissionsOverwritesConfig>>,
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
            permissions_overwrites: permissions_overwrites.compress(),
            extra_channels: ChannelExtraItemsConfig::default(),
        }
    }
}

impl CategoryConfig {
    pub fn into(self, roles: &RolesList<AwaitingRole>) -> AwaitingCategory {
        let overwrites = self
            .permissions_overwrites
            .map(|permissions| {
                permissions
                    .into_iter()
                    .map(|permission| permission.into(roles))
                    .collect::<Vec<PermissionsOverwrites<AwaitingRole>>>()
            })
            .unwrap_or_default();

        AwaitingCategory {
            name: self.name,
            overwrites: overwrites.into(),
            extra_channels_strategy: self.extra_channels.strategy.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use disma::{
        category::{AwaitingCategory, ExistingCategory},
        permission::{
            Permission, PermissionsList, PermissionsOverwrites, PermissionsOverwritesList,
        },
        role::{AwaitingRole, ExistingRole, RolesList},
    };

    use crate::infra::config::channel::{ChannelExtraItemsConfig, ChannelExtraItemsStrategy};

    use super::{CategoryConfig, PermissionsOverwritesConfig};

    fn given_awaiting_roles(names: Vec<&str>) -> RolesList<AwaitingRole> {
        let roles: Vec<AwaitingRole> = names.iter().map(|name| given_awaiting_role(name)).collect();
        RolesList::from(roles)
    }

    fn given_awaiting_role(name: &str) -> AwaitingRole {
        let permissions: Vec<String> = vec![];
        AwaitingRole {
            name: name.to_string(),
            permissions: PermissionsList::from(permissions),
            color: None,
            is_mentionable: true,
            show_in_sidebar: false,
        }
    }

    fn given_existing_role(id: &str, name: &str) -> ExistingRole {
        let permissions: Vec<String> = vec![];
        ExistingRole {
            id: id.to_string(),
            name: name.to_string(),
            permissions: PermissionsList::from(permissions),
            color: None,
            is_mentionable: true,
            show_in_sidebar: false,
        }
    }

    #[test]
    fn can_convert_config_to_awaiting_entity() {
        let category_name = "presto".to_string();
        let role_name = "Team01";
        let roles = given_awaiting_roles(vec![role_name]);
        let role = given_awaiting_role(role_name);

        let config = CategoryConfig {
            name: category_name.clone(),
            permissions_overwrites: Some(vec![PermissionsOverwritesConfig {
                role: role_name.to_string(),
                allow: Some(vec!["ADMINISTRATOR".to_string()]),
                deny: Some(vec!["ADMINISTRATOR".to_string()]),
            }]),
            extra_channels: ChannelExtraItemsConfig {
                strategy: ChannelExtraItemsStrategy::Remove,
            },
        };

        let entity: AwaitingCategory = config.into(&roles);

        let expected_entity = AwaitingCategory {
            name: category_name.clone(),
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrites {
                role,
                allow: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                deny: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
            }]),
            extra_channels_strategy: ChannelExtraItemsConfig::default().strategy.into(),
        };
        assert_eq!(entity, expected_entity);
    }

    #[test]
    fn can_convert_compressed_config_to_awaiting_entity() {
        let category_name = "presto".to_string();

        let config = CategoryConfig {
            name: category_name.clone(),
            permissions_overwrites: None,
            extra_channels: ChannelExtraItemsConfig {
                strategy: ChannelExtraItemsStrategy::Remove,
            },
        };

        let entity: AwaitingCategory = config.into(&RolesList::from(vec![]));

        let expected_entity = AwaitingCategory {
            name: category_name.clone(),
            overwrites: PermissionsOverwritesList::from(vec![]),
            extra_channels_strategy: ChannelExtraItemsConfig::default().strategy.into(),
        };
        assert_eq!(entity, expected_entity);
    }

    #[test]
    fn can_convert_existing_entity_to_config() {
        let category_name = "presto".to_string();
        let role_id = "kgj399sd";
        let role_name = "Team01";
        let role = given_existing_role(role_id, role_name);

        let entity = ExistingCategory {
            id: "some".to_string(),
            name: category_name.clone(),
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrites {
                role,
                allow: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                deny: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
            }]),
        };

        let config = CategoryConfig::from(&entity);

        let expected_config = CategoryConfig {
            name: category_name.clone(),
            permissions_overwrites: Some(vec![PermissionsOverwritesConfig {
                role: role_name.to_string(),
                allow: Some(vec!["ADMINISTRATOR".to_string()]),
                deny: Some(vec!["ADMINISTRATOR".to_string()]),
            }]),
            extra_channels: ChannelExtraItemsConfig {
                strategy: ChannelExtraItemsStrategy::Remove,
            },
        };
        assert_eq!(config, expected_config);
    }

    #[test]
    fn can_convert_existing_entity_to_compressed_config() {
        let category_name = "presto".to_string();

        let entity = ExistingCategory {
            id: "some".to_string(),
            name: category_name.clone(),
            overwrites: PermissionsOverwritesList::from(vec![]),
        };

        let config = CategoryConfig::from(&entity);

        let expected_config = CategoryConfig {
            name: category_name.clone(),
            permissions_overwrites: None,
            extra_channels: ChannelExtraItemsConfig {
                strategy: ChannelExtraItemsStrategy::Remove,
            },
        };
        assert_eq!(config, expected_config);
    }
}
