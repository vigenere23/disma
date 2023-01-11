use disma::role::{ExistingRole, RolesList};

use super::{RoleConfig, RoleConfigsList};

impl From<&RolesList<ExistingRole>> for RoleConfigsList {
    fn from(roles: &RolesList<ExistingRole>) -> Self {
        let items = roles
            .to_list()
            .into_iter()
            .map(|category| category.into())
            .collect();

        RoleConfigsList {
            items,
            ..Default::default()
        }
    }
}

impl From<&ExistingRole> for RoleConfig {
    fn from(role: &ExistingRole) -> Self {
        Self {
            name: role.name.clone(),
            permissions: role.permissions.to_list(),
            color: role.color.clone(),
            show_in_sidebar: role.show_in_sidebar,
            is_mentionable: role.is_mentionable,
        }
    }
}

#[cfg(test)]
mod tests {
    use disma::{
        permission::{Permission, PermissionsList},
        role::{ExistingRole, RolesList},
    };

    use crate::infra::config::role::{RoleConfig, RoleConfigsList};

    fn given_matching_existing_and_config(name: &str) -> (ExistingRole, RoleConfig) {
        let existing = ExistingRole {
            id: "something".to_string(),
            name: name.to_string(),
            color: Some("826d5f".to_string()),
            is_mentionable: true,
            show_in_sidebar: false,
            permissions: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
        };

        let config = RoleConfig {
            name: name.to_string(),
            color: Some("826d5f".to_string()),
            is_mentionable: true,
            show_in_sidebar: false,
            permissions: vec![Permission::ADMINISTRATOR],
        };

        (existing, config)
    }

    fn given_matching_existing_list_and_config_list(
        name: &str,
    ) -> (RolesList<ExistingRole>, RoleConfigsList) {
        let (existing_item, config_item) = given_matching_existing_and_config(name);

        let existing_list = RolesList::from(vec![existing_item]);

        let config_list = RoleConfigsList {
            items: vec![config_item],
            ..Default::default()
        };

        (existing_list, config_list)
    }

    #[test]
    fn can_convert_existing_to_config() {
        let name = "Team10";
        let (existing, expected_config) = given_matching_existing_and_config(name);

        let config = RoleConfig::from(&existing);

        assert_eq!(config, expected_config);
    }

    #[test]
    fn can_convert_existing_entities_list_to_config_list() {
        let name = "presto";
        let (existing_list, expected_config_list) =
            given_matching_existing_list_and_config_list(name);

        let config_list = RoleConfigsList::from(&existing_list);

        assert_eq!(config_list, expected_config_list);
    }
}
