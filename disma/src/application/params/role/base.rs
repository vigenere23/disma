use serde::{Deserialize, Serialize};

use crate::permission::Permission;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct RolesParamsList {
    #[serde(default = "Vec::default")]
    pub items: Vec<RoleParams>,
    #[serde(default = "RoleParamsExtraItemsStrategy::default")]
    pub extra_items: RoleParamsExtraItemsStrategy,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "strategy")]
pub enum RoleParamsExtraItemsStrategy {
    KEEP,
    REMOVE,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RoleParams {
    pub name: String,
    #[serde(default = "Vec::default")]
    pub permissions: Vec<Permission>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub show_in_sidebar: bool,
    pub is_mentionable: bool,
}

impl Default for RoleParamsExtraItemsStrategy {
    fn default() -> Self {
        Self::REMOVE
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        params::role::{RoleParams, RoleParamsExtraItemsStrategy, RolesParamsList},
        permission::Permission,
    };

    #[test]
    fn it_parses_params_list() {
        let yaml_params_list = r"
            items:
            - name: role_1
              color: 29a1f4
              permissions:
              - ADMINISTRATOR
              - SEND_MESSAGES
              show_in_sidebar: true
              is_mentionable: false
            extra_items:
              strategy: KEEP
        ";
        let expected_params_list = RolesParamsList {
            items: vec![RoleParams {
                name: "role_1".to_string(),
                permissions: vec![Permission::ADMINISTRATOR, Permission::SEND_MESSAGES],
                color: Some("29a1f4".to_string()),
                show_in_sidebar: true,
                is_mentionable: false,
            }],
            extra_items: RoleParamsExtraItemsStrategy::KEEP,
        };

        let params_list: RolesParamsList = serde_yaml::from_str(yaml_params_list).unwrap();

        assert_eq!(params_list, expected_params_list);
    }

    #[test]
    fn it_parses_empty_params_list_to_defaults() {
        let yaml_params_list = r"";

        let params_list: RolesParamsList = serde_yaml::from_str(yaml_params_list).unwrap();

        assert_eq!(params_list, RolesParamsList::default());
    }

    #[test]
    fn it_parses_empty_params_fields_to_defaults() {
        let yaml_params_list = r"
            items:
            - name: role_1
              show_in_sidebar: true
              is_mentionable: false
        ";
        let expected_params_list = RolesParamsList {
            items: vec![RoleParams {
                name: "role_1".to_string(),
                permissions: vec![],
                color: None,
                show_in_sidebar: true,
                is_mentionable: false,
            }],
            extra_items: RoleParamsExtraItemsStrategy::REMOVE,
        };

        let params_list: RolesParamsList = serde_yaml::from_str(yaml_params_list).unwrap();

        assert_eq!(params_list, expected_params_list);
    }
}
