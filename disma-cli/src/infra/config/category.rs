use serde::{Deserialize, Serialize};

use disma::{
    category::{AwaitingCategory, ExistingCategory},
    overwrites::PermissionsOverwrites,
    role::{AwaitingRole, RolesList},
    utils::vec::Compress,
};

use super::permission::PermissionsOverwritesConfig;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CategoryConfig {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_overwrites: Option<Vec<PermissionsOverwritesConfig>>,
    pub allow_extra_channels: bool,
}

impl From<&ExistingCategory> for CategoryConfig {
    fn from(category: &ExistingCategory) -> Self {
        let permissions_overwrites: Vec<PermissionsOverwritesConfig> = category
            .overwrites
            .items()
            .iter()
            .map(PermissionsOverwritesConfig::from)
            .collect();

        Self {
            name: category.name.clone(),
            permissions_overwrites: permissions_overwrites.compress(),
            allow_extra_channels: false,
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
            allow_extra_channels: self.allow_extra_channels,
        }
    }
}

#[cfg(test)]
mod tests {
    use disma::{
        category::{AwaitingCategory, ExistingCategory},
        overwrites::{PermissionsOverwrites, PermissionsOverwritesList},
        permission::{Permission, PermissionsList},
        role::{AwaitingRole, ExistingRole, RolesList},
    };

    use super::{CategoryConfig, PermissionsOverwritesConfig};

    fn given_awaiting_roles(names: Vec<&str>) -> RolesList<AwaitingRole> {
        let roles: Vec<AwaitingRole> = names.iter().map(|name| given_awaiting_role(name)).collect();
        RolesList::from(roles)
    }

    fn given_awaiting_role(name: &str) -> AwaitingRole {
        let permissions: Vec<String> = vec![];
        AwaitingRole {
            name: name.to_string(),
            permissions: PermissionsList::from(&permissions),
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
            permissions: PermissionsList::from(&permissions),
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
            allow_extra_channels: false,
        };

        let entity: AwaitingCategory = config.into(&roles);

        let expected_entity = AwaitingCategory {
            name: category_name.clone(),
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrites {
                role,
                allow: PermissionsList::from(&vec![Permission::ADMINISTRATOR]),
                deny: PermissionsList::from(&vec![Permission::ADMINISTRATOR]),
            }]),
            allow_extra_channels: false,
        };
        assert_eq!(entity, expected_entity);
    }

    #[test]
    fn can_convert_config_to_awaiting_entity_with_optionals() {
        let category_name = "presto".to_string();

        let config = CategoryConfig {
            name: category_name.clone(),
            permissions_overwrites: None,
            allow_extra_channels: false,
        };

        let entity: AwaitingCategory = config.into(&RolesList::from(vec![]));

        let expected_entity = AwaitingCategory {
            name: category_name.clone(),
            overwrites: PermissionsOverwritesList::from(vec![]),
            allow_extra_channels: false,
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
                allow: PermissionsList::from(&vec![Permission::ADMINISTRATOR]),
                deny: PermissionsList::from(&vec![Permission::ADMINISTRATOR]),
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
            allow_extra_channels: false,
        };
        assert_eq!(config, expected_config);
    }

    #[test]
    fn can_convert_existing_entity_to_config_with_optionals() {
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
            allow_extra_channels: false,
        };
        assert_eq!(config, expected_config);
    }
}
