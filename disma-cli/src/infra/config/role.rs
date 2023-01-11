use std::{str::FromStr, sync::Arc};

use serde::{Deserialize, Serialize};

use disma::{
    permission::{Permission, PermissionsList},
    role::{
        AwaitingRole, AwaitingRolesList, ExistingRole, ExtraRolesStrategy, KeepExtraRoles,
        RemoveExtraRoles,
    },
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct RoleConfigsList {
    #[serde(default = "Vec::default")]
    pub items: Vec<RoleConfig>,
    #[serde(default = "RoleExtraItemsConfig::default")]
    pub extra_items: RoleExtraItemsConfig,
}

impl RoleConfigsList {
    pub fn into(self) -> AwaitingRolesList {
        let items = self
            .items
            .into_iter()
            .map(|role| role.into())
            .collect::<Vec<AwaitingRole>>()
            .into();

        AwaitingRolesList {
            items,
            extra_items_strategy: self.extra_items.strategy.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RoleExtraItemsConfig {
    pub strategy: RoleExtraItemsStrategy,
}

impl Default for RoleExtraItemsConfig {
    fn default() -> Self {
        Self {
            strategy: RoleExtraItemsStrategy::REMOVE,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum RoleExtraItemsStrategy {
    KEEP,
    REMOVE,
    // TODO Overwrite,
}

impl Into<Arc<dyn ExtraRolesStrategy>> for RoleExtraItemsStrategy {
    fn into(self) -> Arc<dyn ExtraRolesStrategy> {
        match self {
            RoleExtraItemsStrategy::KEEP => Arc::from(KeepExtraRoles {}),
            RoleExtraItemsStrategy::REMOVE => Arc::from(RemoveExtraRoles {}),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RoleConfig {
    pub name: String,
    #[serde(default = "Vec::default")]
    pub permissions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub show_in_sidebar: bool,
    pub is_mentionable: bool,
}

impl From<&ExistingRole> for RoleConfig {
    fn from(role: &ExistingRole) -> Self {
        let permissions: Vec<String> = role
            .permissions
            .items()
            .iter()
            .map(|permission| permission.to_string())
            .collect();

        Self {
            name: role.name.clone(),
            permissions,
            color: role.color.clone(),
            show_in_sidebar: role.show_in_sidebar,
            is_mentionable: role.is_mentionable,
        }
    }
}

impl Into<AwaitingRole> for RoleConfig {
    fn into(self) -> AwaitingRole {
        let permissions: Vec<Permission> = self
            .permissions
            .iter()
            .map(|permission| Permission::from_str(permission).unwrap())
            .collect();

        AwaitingRole {
            name: self.name,
            permissions: PermissionsList::from(permissions),
            color: self.color.map(|color| color.to_lowercase()),
            is_mentionable: self.is_mentionable,
            show_in_sidebar: self.show_in_sidebar,
        }
    }
}

#[cfg(test)]
mod test {

    mod into_awaiting {
        use disma::{
            permission::{Permission, PermissionsList},
            role::AwaitingRole,
        };

        use crate::infra::config::role::RoleConfig;

        #[test]
        fn can_convert_config_to_awaiting_entity() {
            let is_mentionable = true;
            let show_in_sidebar = false;
            let color = "826d5f".to_string();
            let name = "Team10".to_string();

            let config = RoleConfig {
                name: name.clone(),
                color: Some(color.clone()),
                show_in_sidebar,
                is_mentionable,
                permissions: vec!["ADMINISTRATOR".to_string()],
            };

            let entity: AwaitingRole = config.into();

            let expected_entity = AwaitingRole {
                name: name.clone(),
                color: Some(color.clone()),
                is_mentionable,
                show_in_sidebar,
                permissions: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
            };
            assert_eq!(entity, expected_entity);
        }

        #[test]
        fn can_convert_compressed_config_to_awaiting_entity() {
            let is_mentionable = true;
            let show_in_sidebar = false;
            let name = "Team10".to_string();
            let permissions: Vec<String> = vec![];

            let config = RoleConfig {
                name: name.clone(),
                color: None,
                is_mentionable,
                show_in_sidebar,
                permissions: vec![],
            };

            let entity: AwaitingRole = config.into();

            let expected_entity = AwaitingRole {
                name: name.clone(),
                color: None,
                is_mentionable,
                show_in_sidebar,
                permissions: PermissionsList::from(permissions),
            };
            assert_eq!(entity, expected_entity);
        }
    }

    mod from_existing {
        use disma::{
            permission::{Permission, PermissionsList},
            role::ExistingRole,
        };

        use crate::infra::config::role::RoleConfig;

        #[test]
        fn can_convert_existing_entity_to_config() {
            let is_mentionable = true;
            let show_in_sidebar = false;
            let color = "826d5f".to_string();
            let name = "Team10".to_string();
            let id = "93jdi0".to_string();

            let entity = ExistingRole {
                id: id.clone(),
                name: name.clone(),
                color: Some(color.clone()),
                is_mentionable,
                show_in_sidebar,
                permissions: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
            };

            let config = RoleConfig::from(&entity);

            let expected_config = RoleConfig {
                name: name.clone(),
                color: Some(color.clone()),
                show_in_sidebar,
                is_mentionable,
                permissions: vec!["ADMINISTRATOR".to_string()],
            };
            assert_eq!(config, expected_config);
        }

        #[test]
        fn can_convert_existing_entity_to_compressed_config() {
            let is_mentionable = true;
            let show_in_sidebar = false;
            let name = "Team10".to_string();
            let id = "93jdi0".to_string();
            let permissions: Vec<String> = vec![];

            let entity = ExistingRole {
                id: id.clone(),
                name: name.clone(),
                color: None,
                is_mentionable,
                show_in_sidebar,
                permissions: PermissionsList::from(permissions),
            };

            let config = RoleConfig::from(&entity);

            let expected_config = RoleConfig {
                name: name.clone(),
                color: None,
                show_in_sidebar,
                is_mentionable,
                permissions: vec![],
            };
            assert_eq!(config, expected_config);
        }
    }

    mod serde_parsing {
        use crate::infra::config::role::{
            RoleConfig, RoleConfigsList, RoleExtraItemsConfig, RoleExtraItemsStrategy,
        };

        #[test]
        fn can_be_parsed() {
            let yaml_config = r"
                items:
                  - name: role_1
                    color: 29a1f4
                    permissions:
                      - ADMINISTRATOR
                      - SEND_MESSAGES
                    show_in_sidebar: true
                    is_mentionable: false
                extra_items:
                  strategy: KEEP
            ";
            let expected_config = RoleConfigsList {
                items: vec![RoleConfig {
                    name: "role_1".to_string(),
                    permissions: vec!["ADMINISTRATOR".to_string(), "SEND_MESSAGES".to_string()],
                    color: Some("29a1f4".to_string()),
                    show_in_sidebar: true,
                    is_mentionable: false,
                }],
                extra_items: RoleExtraItemsConfig {
                    strategy: RoleExtraItemsStrategy::KEEP,
                },
            };

            let config: RoleConfigsList = serde_yaml::from_str(yaml_config).unwrap();

            assert_eq!(config, expected_config);
        }

        #[test]
        fn it_fills_empty_config_with_defaults() {
            let yaml_config = r"";

            let config: RoleConfigsList = serde_yaml::from_str(yaml_config).unwrap();

            assert_eq!(config, RoleConfigsList::default());
        }

        #[test]
        fn it_fills_empty_fields_with_defaults() {
            let yaml_config = r"
                items:
                  - name: role_1
                    show_in_sidebar: true
                    is_mentionable: false
            ";
            let expected_config = RoleConfigsList {
                items: vec![RoleConfig {
                    name: "role_1".to_string(),
                    permissions: vec![],
                    color: None,
                    show_in_sidebar: true,
                    is_mentionable: false,
                }],
                ..Default::default()
            };

            let config: RoleConfigsList = serde_yaml::from_str(yaml_config).unwrap();

            assert_eq!(config, expected_config);
        }
    }
}
