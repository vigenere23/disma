use serde::{Deserialize, Serialize};

use crate::params::{channel::ChannelParamsExtraItems, permission::PermissionsOverwriteParams};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct CategoriesParamsList {
    #[serde(default = "Vec::default")]
    pub items: Vec<CategoryParams>,
    #[serde(default = "CategoryParamsExtraItems::default")]
    pub extra_items: CategoryParamsExtraItems,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CategoryParamsExtraItems {
    pub strategy: CategoryParamsExtraItemsStrategy,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum CategoryParamsExtraItemsStrategy {
    KEEP,
    REMOVE,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CategoryParams {
    pub name: String,
    #[serde(default = "Vec::default")]
    pub permissions_overwrites: Vec<PermissionsOverwriteParams>,
    #[serde(default = "ChannelParamsExtraItems::default")]
    pub extra_channels: ChannelParamsExtraItems,
}

impl Default for CategoryParamsExtraItems {
    fn default() -> Self {
        Self {
            strategy: CategoryParamsExtraItemsStrategy::REMOVE,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        params::{
            category::{
                CategoriesParamsList, CategoryParams, CategoryParamsExtraItems,
                CategoryParamsExtraItemsStrategy,
            },
            channel::{ChannelParamsExtraItems, ChannelParamsExtraItemsStrategy},
            permission::PermissionsOverwriteParams,
        },
        permission::Permission,
    };

    #[test]
    fn it_parses_params_list() {
        let yaml_params_list = r"
            items:
            - name: category_1
              permissions_overwrites:
              - role: role_1
                allow: [ADMINISTRATOR]
                deny: [SEND_MESSAGES]
              extra_channels:
                strategy: KEEP
            extra_items:
              strategy: KEEP
        ";
        let expected_params_list = CategoriesParamsList {
            items: vec![CategoryParams {
                name: "category_1".to_string(),
                permissions_overwrites: vec![PermissionsOverwriteParams {
                    role: "role_1".to_string(),
                    allow: vec![Permission::ADMINISTRATOR],
                    deny: vec![Permission::SEND_MESSAGES],
                }],
                extra_channels: ChannelParamsExtraItems {
                    strategy: ChannelParamsExtraItemsStrategy::KEEP,
                },
            }],
            extra_items: CategoryParamsExtraItems {
                strategy: CategoryParamsExtraItemsStrategy::KEEP,
            },
        };

        let params_list: CategoriesParamsList = serde_yaml::from_str(yaml_params_list).unwrap();

        assert_eq!(params_list, expected_params_list);
    }

    #[test]
    fn it_parses_empty_params_list_to_defaults() {
        let yaml_params_list = r"";

        let params_list: CategoriesParamsList = serde_yaml::from_str(yaml_params_list).unwrap();

        assert_eq!(params_list, CategoriesParamsList::default());
    }

    #[test]
    fn it_parses_empty_params_fields_to_defaults() {
        let yaml_params_list = r"
            items:
            - name: category_1
        ";
        let expected_params_list = CategoriesParamsList {
            items: vec![CategoryParams {
                name: "category_1".to_string(),
                permissions_overwrites: vec![],
                extra_channels: ChannelParamsExtraItems::default(),
            }],
            extra_items: CategoryParamsExtraItems {
                strategy: CategoryParamsExtraItemsStrategy::REMOVE,
            },
        };

        let params_list: CategoriesParamsList = serde_yaml::from_str(yaml_params_list).unwrap();

        assert_eq!(params_list, expected_params_list);
    }
}
