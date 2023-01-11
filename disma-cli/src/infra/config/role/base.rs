use serde::{Deserialize, Serialize};

use disma::permission::Permission;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct RoleConfigsList {
    #[serde(default = "Vec::default")]
    pub items: Vec<RoleConfig>,
    #[serde(default = "RoleExtraItemsConfig::default")]
    pub extra_items: RoleExtraItemsConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RoleExtraItemsConfig {
    pub strategy: RoleExtraItemsStrategy,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum RoleExtraItemsStrategy {
    KEEP,
    REMOVE,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RoleConfig {
    pub name: String,
    #[serde(default = "Vec::default")]
    pub permissions: Vec<Permission>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub show_in_sidebar: bool,
    pub is_mentionable: bool,
}

impl Default for RoleExtraItemsConfig {
    fn default() -> Self {
        Self {
            strategy: RoleExtraItemsStrategy::REMOVE,
        }
    }
}

#[cfg(test)]
mod tests {
    use disma::permission::Permission;

    use crate::infra::config::role::{
        RoleConfig, RoleConfigsList, RoleExtraItemsConfig, RoleExtraItemsStrategy,
    };

    #[test]
    fn is_parses_config_list() {
        let yaml_config = r"
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
        let expected_config = RoleConfigsList {
            items: vec![RoleConfig {
                name: "role_1".to_string(),
                permissions: vec![Permission::ADMINISTRATOR, Permission::SEND_MESSAGES],
                color: Some("29a1f4".to_string()),
                show_in_sidebar: true,
                is_mentionable: false,
            }],
            extra_items: RoleExtraItemsConfig {
                strategy: RoleExtraItemsStrategy::KEEP,
            },
        };

        let config: RoleConfigsList = serde_yaml::from_str(yaml_config).unwrap();

        assert_eq!(config, expected_config);
    }

    #[test]
    fn it_parses_empty_config_list_to_defaults() {
        let yaml_config = r"";

        let config: RoleConfigsList = serde_yaml::from_str(yaml_config).unwrap();

        assert_eq!(config, RoleConfigsList::default());
    }

    #[test]
    fn it_parses_empty_config_fields_to_defaults() {
        let yaml_config = r"
            items:
            - name: role_1
              show_in_sidebar: true
              is_mentionable: false
        ";
        let expected_config = RoleConfigsList {
            items: vec![RoleConfig {
                name: "role_1".to_string(),
                permissions: vec![],
                color: None,
                show_in_sidebar: true,
                is_mentionable: false,
            }],
            ..Default::default()
        };

        let config: RoleConfigsList = serde_yaml::from_str(yaml_config).unwrap();

        assert_eq!(config, expected_config);
    }
}
