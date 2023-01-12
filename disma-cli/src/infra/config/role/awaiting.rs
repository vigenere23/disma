use std::sync::Arc;

use disma::{
    permission::PermissionsList,
    role::{AwaitingRole, AwaitingRolesList, ExtraRolesStrategy, KeepExtraRoles, RemoveExtraRoles},
};

use super::{RoleConfig, RoleConfigsList, RoleExtraItemsStrategy};

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

impl Into<Arc<dyn ExtraRolesStrategy>> for RoleExtraItemsStrategy {
    fn into(self) -> Arc<dyn ExtraRolesStrategy> {
        match self {
            RoleExtraItemsStrategy::KEEP => Arc::from(KeepExtraRoles {}),
            RoleExtraItemsStrategy::REMOVE => Arc::from(RemoveExtraRoles {}),
        }
    }
}

impl Into<AwaitingRole> for RoleConfig {
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

    use disma::{
        permission::{Permission, PermissionsList},
        role::{AwaitingRole, AwaitingRolesList, KeepExtraRoles, RolesList},
    };

    use crate::infra::config::role::{
        RoleConfig, RoleConfigsList, RoleExtraItemsConfig, RoleExtraItemsStrategy,
    };

    fn given_matching_config_and_awaiting(name: &str) -> (RoleConfig, AwaitingRole) {
        let config = RoleConfig {
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

        (config, awaiting)
    }

    fn given_matching_config_list_and_awaiting_list(
        name: &str,
    ) -> (RoleConfigsList, AwaitingRolesList) {
        let (config_item, awaiting_item) = given_matching_config_and_awaiting(name);

        let config_list = RoleConfigsList {
            items: vec![config_item],
            extra_items: RoleExtraItemsConfig {
                strategy: RoleExtraItemsStrategy::KEEP,
            },
        };

        let awaiting_list = AwaitingRolesList {
            items: RolesList::from(vec![awaiting_item]),
            extra_items_strategy: Arc::from(KeepExtraRoles {}),
        };

        (config_list, awaiting_list)
    }

    #[test]
    fn can_convert_config_to_awaiting() {
        let name = "Team10";
        let (config, expected_awaiting) = given_matching_config_and_awaiting(name);

        let awaiting: AwaitingRole = config.into();

        assert_eq!(awaiting, expected_awaiting);
    }

    #[test]
    fn can_convert_compressed_config_to_awaiting_entity() {
        let name = "presto";
        let (config_list, expected_awaiting_list) =
            given_matching_config_list_and_awaiting_list(name);

        let awaiting_list: AwaitingRolesList = config_list.into();

        assert_eq!(awaiting_list, expected_awaiting_list);
    }
}
