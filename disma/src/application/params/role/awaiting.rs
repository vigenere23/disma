use std::sync::Arc;

use crate::{
    permission::PermissionsList,
    role::{AwaitingRole, AwaitingRolesList, ExtraRolesStrategy, KeepExtraRoles, RemoveExtraRoles},
};

use super::{RoleParams, RoleParamsExtraItemsStrategy, RolesParamsList};

impl RolesParamsList {
    pub fn into(self) -> AwaitingRolesList {
        let items = self
            .items
            .into_iter()
            .map(|role| role.into())
            .collect::<Vec<AwaitingRole>>()
            .into();

        AwaitingRolesList {
            items,
            extra_items_strategy: self.extra_items.into(),
        }
    }
}

impl Into<Arc<dyn ExtraRolesStrategy>> for RoleParamsExtraItemsStrategy {
    fn into(self) -> Arc<dyn ExtraRolesStrategy> {
        match self {
            RoleParamsExtraItemsStrategy::Keep => Arc::from(KeepExtraRoles {}),
            RoleParamsExtraItemsStrategy::Remove => Arc::from(RemoveExtraRoles {}),
        }
    }
}

impl Into<AwaitingRole> for RoleParams {
    fn into(self) -> AwaitingRole {
        AwaitingRole {
            name: self.name,
            permissions: PermissionsList::from(self.permissions),
            color: self.color.map(|color| color.to_lowercase()),
            is_mentionable: self.is_mentionable,
            show_in_sidebar: self.show_in_sidebar,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        params::role::{RoleParams, RoleParamsExtraItemsStrategy, RolesParamsList},
        permission::{Permission, PermissionsList},
        role::{AwaitingRole, AwaitingRolesList, KeepExtraRoles, RolesList},
    };

    fn given_matching_params_and_awaiting(name: &str) -> (RoleParams, AwaitingRole) {
        let params = RoleParams {
            name: name.to_string(),
            color: Some("826d5f".to_string()),
            is_mentionable: true,
            show_in_sidebar: false,
            permissions: vec![Permission::ADMINISTRATOR],
        };

        let awaiting = AwaitingRole {
            name: name.to_string(),
            color: Some("826d5f".to_string()),
            is_mentionable: true,
            show_in_sidebar: false,
            permissions: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
        };

        (params, awaiting)
    }

    fn given_matching_params_list_and_awaiting_list(
        name: &str,
    ) -> (RolesParamsList, AwaitingRolesList) {
        let (params, awaiting) = given_matching_params_and_awaiting(name);

        let params_list = RolesParamsList {
            items: vec![params],
            extra_items: RoleParamsExtraItemsStrategy::Keep,
        };

        let awaiting_list = AwaitingRolesList {
            items: RolesList::from(vec![awaiting]),
            extra_items_strategy: Arc::from(KeepExtraRoles {}),
        };

        (params_list, awaiting_list)
    }

    #[test]
    fn can_convert_params_to_awaiting() {
        let name = "Team10";
        let (params, expected_awaiting) = given_matching_params_and_awaiting(name);

        let awaiting: AwaitingRole = params.into();

        assert_eq!(awaiting, expected_awaiting);
    }

    #[test]
    fn can_convert_compressed_params_to_awaiting_entity() {
        let name = "presto";
        let (params_list, expected_awaiting_list) =
            given_matching_params_list_and_awaiting_list(name);

        let awaiting_list: AwaitingRolesList = params_list.into();

        assert_eq!(awaiting_list, expected_awaiting_list);
    }
}
