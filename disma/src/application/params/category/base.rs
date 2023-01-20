use serde::{Deserialize, Serialize};

use crate::params::permission::PermissionsOverwriteParams;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct CategoriesParamsList {
    #[serde(default = "Vec::default")]
    pub items: Vec<CategoryParams>,
    #[serde(default = "CategoryParamsExtraItemsStrategy::default")]
    pub extra_items: CategoryParamsExtraItemsStrategy,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "strategy", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CategoryParamsExtraItemsStrategy {
    Keep,
    Remove,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CategoryParams {
    pub name: String,
    #[serde(default = "Vec::default")]
    pub permissions_overwrites: Vec<PermissionsOverwriteParams>,
    #[serde(default = "CategoryParamsExtraChannelsStrategy::default")]
    pub extra_channels: CategoryParamsExtraChannelsStrategy,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "strategy", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CategoryParamsExtraChannelsStrategy {
    Keep,
    Remove,
    OverwritePermissionsFromCategory,
}

impl Default for CategoryParamsExtraItemsStrategy {
    fn default() -> Self {
        Self::Remove
    }
}

impl Default for CategoryParamsExtraChannelsStrategy {
    fn default() -> Self {
        Self::Remove
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        params::{
            category::{
                CategoriesParamsList, CategoryParams, CategoryParamsExtraChannelsStrategy,
                CategoryParamsExtraItemsStrategy,
            },
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
                extra_channels: CategoryParamsExtraChannelsStrategy::Keep,
            }],
            extra_items: CategoryParamsExtraItemsStrategy::Keep,
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
                extra_channels: CategoryParamsExtraChannelsStrategy::default(),
            }],
            extra_items: CategoryParamsExtraItemsStrategy::Remove,
        };

        let params_list: CategoriesParamsList = serde_yaml::from_str(yaml_params_list).unwrap();

        assert_eq!(params_list, expected_params_list);
    }
}
