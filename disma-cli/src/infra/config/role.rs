use std::str::FromStr;

use serde::{Deserialize, Serialize};

use disma::{
    permission::{Permission, PermissionsList},
    role::{AwaitingRole, ExistingRole},
    utils::vec::Compress,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RoleConfig {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
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
            permissions: permissions.compress(),
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
            .unwrap_or_default()
            .iter()
            .map(|permission| Permission::from_str(permission).unwrap())
            .collect();

        AwaitingRole {
            name: self.name,
            permissions: PermissionsList::from(&permissions),
            color: self.color.map(|color| color.to_lowercase()),
            is_mentionable: self.is_mentionable,
            show_in_sidebar: self.show_in_sidebar,
        }
    }
}

#[cfg(test)]
mod test {
    use disma::{
        permission::{Permission, PermissionsList},
        role::{AwaitingRole, ExistingRole},
    };

    use super::RoleConfig;

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
            permissions: Some(vec!["ADMINISTRATOR".to_string()]),
        };

        let entity: AwaitingRole = config.into();

        let expected_entity = AwaitingRole {
            name: name.clone(),
            color: Some(color.clone()),
            is_mentionable,
            show_in_sidebar,
            permissions: PermissionsList::from(&vec![Permission::ADMINISTRATOR]),
        };
        assert_eq!(entity, expected_entity);
    }

    #[test]
    fn can_convert_config_to_awaiting_entity_with_optionals() {
        let is_mentionable = true;
        let show_in_sidebar = false;
        let name = "Team10".to_string();
        let permissions: Vec<String> = vec![];

        let config = RoleConfig {
            name: name.clone(),
            color: None,
            show_in_sidebar,
            is_mentionable,
            permissions: None,
        };

        let entity: AwaitingRole = config.into();

        let expected_entity = AwaitingRole {
            name: name.clone(),
            color: None,
            is_mentionable,
            show_in_sidebar,
            permissions: PermissionsList::from(&permissions),
        };
        assert_eq!(entity, expected_entity);
    }

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
            permissions: PermissionsList::from(&vec![Permission::ADMINISTRATOR]),
        };

        let config = RoleConfig::from(&entity);

        let expected_config = RoleConfig {
            name: name.clone(),
            color: Some(color.clone()),
            show_in_sidebar,
            is_mentionable,
            permissions: Some(vec!["ADMINISTRATOR".to_string()]),
        };
        assert_eq!(config, expected_config);
    }

    #[test]
    fn can_convert_existing_entity_to_config_with_optionals() {
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
            permissions: PermissionsList::from(&permissions),
        };

        let config = RoleConfig::from(&entity);

        let expected_config = RoleConfig {
            name: name.clone(),
            color: None,
            show_in_sidebar,
            is_mentionable,
            permissions: None,
        };
        assert_eq!(config, expected_config);
    }
}
