use disma::category::{CategoriesList, ExistingCategory};

use crate::infra::config::{
    channel::ChannelExtraItemsConfig, permission::PermissionsOverwritesConfig,
};

use super::{CategoryConfig, CategoryConfigsList};

impl From<&CategoriesList<ExistingCategory>> for CategoryConfigsList {
    fn from(categories: &CategoriesList<ExistingCategory>) -> Self {
        let items = categories.to_list().iter().map(Into::into).collect();

        CategoryConfigsList {
            items,
            ..Default::default()
        }
    }
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
            permissions_overwrites,
            extra_channels: ChannelExtraItemsConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use disma::{
        category::{CategoriesList, ExistingCategory},
        permission::{
            Permission, PermissionsList, PermissionsOverwrites, PermissionsOverwritesList,
        },
        role::ExistingRole,
    };

    use crate::infra::config::{
        category::{CategoryConfig, CategoryConfigsList},
        channel::{ChannelExtraItemsConfig, ChannelExtraItemsStrategy},
        permission::PermissionsOverwritesConfig,
    };

    fn given_matching_existing_and_config(
        name: &str,
        role: &ExistingRole,
    ) -> (ExistingCategory, CategoryConfig) {
        let existing = ExistingCategory {
            id: "some".to_string(),
            name: name.to_string(),
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrites {
                role: role.clone(),
                allow: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                deny: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
            }]),
        };

        let config = CategoryConfig {
            name: name.to_string(),
            permissions_overwrites: vec![PermissionsOverwritesConfig {
                role: role.name.clone(),
                allow: vec![Permission::ADMINISTRATOR],
                deny: vec![Permission::ADMINISTRATOR],
            }],
            extra_channels: ChannelExtraItemsConfig {
                strategy: ChannelExtraItemsStrategy::REMOVE,
            },
        };

        (existing, config)
    }

    fn given_matching_existing_list_and_config_list(
        name: &str,
        role: &ExistingRole,
    ) -> (CategoriesList<ExistingCategory>, CategoryConfigsList) {
        let (existing_item, config_item) = given_matching_existing_and_config(name, role);

        let existing_list = CategoriesList::from(vec![existing_item]);

        let config_list = CategoryConfigsList {
            items: vec![config_item],
            ..Default::default()
        };

        (existing_list, config_list)
    }

    fn given_existing_role(id: &str, name: &str) -> ExistingRole {
        ExistingRole {
            id: id.to_string(),
            name: name.to_string(),
            permissions: PermissionsList::from(vec![]),
            color: None,
            is_mentionable: true,
            show_in_sidebar: false,
        }
    }

    #[test]
    fn can_convert_existing_entity_to_config() {
        let name = "presto";
        let role = given_existing_role("kgj399sd", "Team01");
        let (existing, expected_config) = given_matching_existing_and_config(name, &role);

        let config = CategoryConfig::from(&existing);

        assert_eq!(config, expected_config);
    }

    #[test]
    fn can_convert_existing_entities_list_to_config_list() {
        let name = "presto";
        let role = given_existing_role("kgj399sd", "Team01");
        let (existing_list, expected_config_list) =
            given_matching_existing_list_and_config_list(name, &role);

        let config = CategoryConfigsList::from(&existing_list);

        assert_eq!(config, expected_config_list);
    }
}
