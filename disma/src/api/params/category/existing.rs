use crate::{
    api::params::permission::PermissionsOverwriteParams,
    category::{CategoriesList, ExistingCategory},
};

use super::{CategoriesParamsList, CategoryParams, CategoryParamsExtraChannelsStrategy};

impl From<&CategoriesList<ExistingCategory>> for CategoriesParamsList {
    fn from(categories: &CategoriesList<ExistingCategory>) -> Self {
        let items = categories.to_list().into_iter().map(Into::into).collect();

        CategoriesParamsList {
            items,
            ..Default::default()
        }
    }
}

impl From<&ExistingCategory> for CategoryParams {
    fn from(category: &ExistingCategory) -> Self {
        let permissions_overwrites: Vec<PermissionsOverwriteParams> = category
            .overwrites
            .to_list()
            .iter()
            .map(PermissionsOverwriteParams::from)
            .collect();

        Self {
            name: category.name.clone(),
            permissions_overwrites,
            extra_channels: CategoryParamsExtraChannelsStrategy::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::params::{
            category::{CategoriesParamsList, CategoryParams, CategoryParamsExtraChannelsStrategy},
            permission::PermissionsOverwriteParams,
        },
        category::{CategoriesList, ExistingCategory},
        permission::{
            Permission, PermissionsList, PermissionsOverwrite, PermissionsOverwritesList,
        },
        role::ExistingRole,
    };

    fn given_matching_existing_and_params(
        name: &str,
        role: &ExistingRole,
    ) -> (ExistingCategory, CategoryParams) {
        let existing = ExistingCategory {
            id: "some".to_string(),
            name: name.to_string(),
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrite {
                role: role.clone(),
                allow: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                deny: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
            }]),
        };

        let params = CategoryParams {
            name: name.to_string(),
            permissions_overwrites: vec![PermissionsOverwriteParams {
                role: role.name.clone(),
                allow: vec![Permission::ADMINISTRATOR],
                deny: vec![Permission::ADMINISTRATOR],
            }],
            extra_channels: CategoryParamsExtraChannelsStrategy::Keep,
        };

        (existing, params)
    }

    fn given_matching_existing_list_and_params_list(
        name: &str,
        role: &ExistingRole,
    ) -> (CategoriesList<ExistingCategory>, CategoriesParamsList) {
        let (existing, params) = given_matching_existing_and_params(name, role);

        let existing_list = CategoriesList::from(vec![existing]);

        let params_list = CategoriesParamsList {
            items: vec![params],
            ..Default::default()
        };

        (existing_list, params_list)
    }

    fn given_existing_role(id: &str, name: &str) -> ExistingRole {
        ExistingRole {
            id: id.to_string(),
            name: name.to_string(),
            permissions: PermissionsList::new(),
            color: None,
            is_mentionable: true,
            show_in_sidebar: false,
        }
    }

    #[test]
    fn can_convert_existing_entity_to_params() {
        let name = "presto";
        let role = given_existing_role("kgj399sd", "Team01");
        let (existing, expected_params) = given_matching_existing_and_params(name, &role);

        let params = CategoryParams::from(&existing);

        assert_eq!(params, expected_params);
    }

    #[test]
    fn can_convert_existing_entities_list_to_params_list() {
        let name = "presto";
        let role = given_existing_role("kgj399sd", "Team01");
        let (existing_list, expected_params_list) =
            given_matching_existing_list_and_params_list(name, &role);

        let params = CategoriesParamsList::from(&existing_list);

        assert_eq!(params, expected_params_list);
    }
}
