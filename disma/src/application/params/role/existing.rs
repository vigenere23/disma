use crate::role::{ExistingRole, RolesList};

use super::{RoleParams, RolesParamsList};

impl From<&RolesList<ExistingRole>> for RolesParamsList {
    fn from(roles: &RolesList<ExistingRole>) -> Self {
        let items = roles.to_list().into_iter().map(Into::into).collect();

        RolesParamsList {
            items,
            ..Default::default()
        }
    }
}

impl From<&ExistingRole> for RoleParams {
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
    use crate::{
        params::role::{RoleParams, RolesParamsList},
        permission::{Permission, PermissionsList},
        role::{ExistingRole, RolesList},
    };

    fn given_matching_existing_and_params(name: &str) -> (ExistingRole, RoleParams) {
        let existing = ExistingRole {
            id: "something".to_string(),
            name: name.to_string(),
            color: Some("826d5f".to_string()),
            is_mentionable: true,
            show_in_sidebar: false,
            permissions: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
        };

        let params = RoleParams {
            name: name.to_string(),
            color: Some("826d5f".to_string()),
            is_mentionable: true,
            show_in_sidebar: false,
            permissions: vec![Permission::ADMINISTRATOR],
        };

        (existing, params)
    }

    fn given_matching_existing_list_and_params_list(
        name: &str,
    ) -> (RolesList<ExistingRole>, RolesParamsList) {
        let (existing, params) = given_matching_existing_and_params(name);

        let existing_list = RolesList::from(vec![existing]);

        let params_list = RolesParamsList {
            items: vec![params],
            ..Default::default()
        };

        (existing_list, params_list)
    }

    #[test]
    fn can_convert_existing_to_params() {
        let name = "Team10";
        let (existing, expected_params) = given_matching_existing_and_params(name);

        let params = RoleParams::from(&existing);

        assert_eq!(params, expected_params);
    }

    #[test]
    fn can_convert_existing_entities_list_to_params_list() {
        let name = "presto";
        let (existing_list, expected_params_list) =
            given_matching_existing_list_and_params_list(name);

        let params_list = RolesParamsList::from(&existing_list);

        assert_eq!(params_list, expected_params_list);
    }
}
