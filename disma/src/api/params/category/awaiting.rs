use std::sync::Arc;

use crate::{
    category::{
        AwaitingCategoriesList, AwaitingCategory, ExtraCategoriesStrategy, KeepExtraCategories,
        RemoveExtraCategories,
    },
    channel::{
        ExtraChannelsStrategy, KeepExtraChannels, RemoveExtraChannels, SyncExtraChannelsPermissions,
    },
    permission::PermissionsOverwrite,
    role::{AwaitingRole, RolesList},
};

use super::{
    CategoriesParamsList, CategoryParams, CategoryParamsExtraChannelsStrategy,
    CategoryParamsExtraItemsStrategy,
};

impl CategoriesParamsList {
    pub fn into(self, roles: &RolesList<AwaitingRole>) -> AwaitingCategoriesList {
        let items = self
            .items
            .into_iter()
            .map(|category| category.into(roles))
            .collect::<Vec<AwaitingCategory>>()
            .into();

        AwaitingCategoriesList {
            items,
            extra_items_strategy: self.extra_items.into(),
        }
    }
}

impl Into<Arc<dyn ExtraCategoriesStrategy>> for CategoryParamsExtraItemsStrategy {
    fn into(self) -> Arc<dyn ExtraCategoriesStrategy> {
        match self {
            Self::Keep => Arc::from(KeepExtraCategories {}),
            Self::Remove => Arc::from(RemoveExtraCategories {}),
        }
    }
}

impl CategoryParams {
    pub fn into(self, roles: &RolesList<AwaitingRole>) -> AwaitingCategory {
        let overwrites = self
            .permissions_overwrites
            .into_iter()
            .map(|permission| permission.into(roles))
            .collect::<Vec<PermissionsOverwrite<AwaitingRole>>>();

        AwaitingCategory {
            name: self.name,
            overwrites: overwrites.into(),
            extra_channels_strategy: self.extra_channels.into(),
        }
    }
}

impl CategoryParamsExtraChannelsStrategy {
    pub fn into(self) -> Arc<dyn ExtraChannelsStrategy> {
        match self {
            Self::Keep => Arc::from(KeepExtraChannels {}),
            Self::Remove => Arc::from(RemoveExtraChannels {}),
            Self::SyncPermissions => Arc::from(SyncExtraChannelsPermissions {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        api::params::{
            category::{
                CategoriesParamsList, CategoryParams, CategoryParamsExtraChannelsStrategy,
                CategoryParamsExtraItemsStrategy,
            },
            permission::PermissionsOverwriteParams,
        },
        category::{AwaitingCategoriesList, AwaitingCategory, CategoriesList, KeepExtraCategories},
        channel::KeepExtraChannels,
        permission::{
            Permission, PermissionsList, PermissionsOverwrite, PermissionsOverwritesList,
        },
        role::{AwaitingRole, RolesList},
    };

    fn given_matching_params_and_awaiting(
        name: &str,
        roles: &RolesList<AwaitingRole>,
    ) -> (CategoryParams, AwaitingCategory) {
        let role = roles.to_list().first().cloned().unwrap();

        let params = CategoryParams {
            name: name.to_string(),
            permissions_overwrites: vec![PermissionsOverwriteParams {
                role: role.name.clone(),
                allow: vec![Permission::ADMINISTRATOR],
                deny: vec![Permission::ADMINISTRATOR],
            }],
            extra_channels: CategoryParamsExtraChannelsStrategy::Keep,
        };

        let awaiting_entity = AwaitingCategory {
            name: name.to_string(),
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrite {
                role: role.clone(),
                allow: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                deny: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
            }]),
            extra_channels_strategy: Arc::from(KeepExtraChannels {}),
        };

        (params, awaiting_entity)
    }

    fn given_matching_params_list_and_awaiting_list(
        name: &str,
        roles: &RolesList<AwaitingRole>,
    ) -> (CategoriesParamsList, AwaitingCategoriesList) {
        let (params, awaiting) = given_matching_params_and_awaiting(name, roles);

        let params_list = CategoriesParamsList {
            items: vec![params],
            extra_items: CategoryParamsExtraItemsStrategy::Keep,
        };

        let awaiting_list = AwaitingCategoriesList {
            items: CategoriesList::from(vec![awaiting]),
            extra_items_strategy: Arc::from(KeepExtraCategories {}),
        };

        (params_list, awaiting_list)
    }

    fn given_awaiting_roles(names: Vec<&str>) -> RolesList<AwaitingRole> {
        let roles: Vec<AwaitingRole> = names.iter().map(|name| given_awaiting_role(name)).collect();
        RolesList::from(roles)
    }

    fn given_awaiting_role(name: &str) -> AwaitingRole {
        AwaitingRole {
            name: name.to_string(),
            permissions: PermissionsList::new(),
            color: None,
            is_mentionable: true,
            show_in_sidebar: false,
        }
    }

    #[test]
    fn can_convert_params_to_awaiting_entity() {
        let name = "category_1";
        let roles = given_awaiting_roles(vec!["role_1"]);
        let (params, expected_awaiting) = given_matching_params_and_awaiting(name, &roles);

        let awaiting: AwaitingCategory = params.into(&roles);

        assert_eq!(awaiting, expected_awaiting);
    }

    #[test]
    fn can_convert_params_list_to_awaiting_entity_list() {
        let name = "category_1";
        let roles = given_awaiting_roles(vec!["role_1"]);
        let (params_list, expected_awaiting_list) =
            given_matching_params_list_and_awaiting_list(name, &roles);

        let awaiting_list: AwaitingCategoriesList = params_list.into(&roles);

        assert_eq!(awaiting_list, expected_awaiting_list);
    }
}
