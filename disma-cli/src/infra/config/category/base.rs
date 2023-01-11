use serde::{Deserialize, Serialize};

use crate::infra::config::{
    channel::ChannelExtraItemsConfig, permission::PermissionsOverwritesConfig,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CategoryConfigsList {
    #[serde(default = "Vec::default")]
    pub items: Vec<CategoryConfig>,
    #[serde(default = "CategoryExtraItemsConfig::default")]
    pub extra_items: CategoryExtraItemsConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CategoryExtraItemsConfig {
    pub strategy: CategoryExtraItemsStrategy,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CategoryExtraItemsStrategy {
    KEEP,
    REMOVE,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CategoryConfig {
    pub name: String,
    #[serde(default = "Vec::default")]
    pub permissions_overwrites: Vec<PermissionsOverwritesConfig>,
    #[serde(default = "ChannelExtraItemsConfig::default")]
    pub extra_channels: ChannelExtraItemsConfig,
}

impl Default for CategoryExtraItemsConfig {
    fn default() -> Self {
        Self {
            strategy: CategoryExtraItemsStrategy::REMOVE,
        }
    }
}

#[cfg(test)]
mod tests {
    use disma::permission::Permission;

    use crate::infra::config::{
        category::{
            CategoryConfig, CategoryConfigsList, CategoryExtraItemsConfig,
            CategoryExtraItemsStrategy,
        },
        channel::{ChannelExtraItemsConfig, ChannelExtraItemsStrategy},
        permission::PermissionsOverwritesConfig,
    };

    #[test]
    fn is_parses_config_list() {
        let yaml_config = r"
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
        let expected_config = CategoryConfigsList {
            items: vec![CategoryConfig {
                name: "category_1".to_string(),
                permissions_overwrites: vec![PermissionsOverwritesConfig {
                    role: "role_1".to_string(),
                    allow: vec![Permission::ADMINISTRATOR],
                    deny: vec![Permission::SEND_MESSAGES],
                }],
                extra_channels: ChannelExtraItemsConfig {
                    strategy: ChannelExtraItemsStrategy::KEEP,
                },
            }],
            extra_items: CategoryExtraItemsConfig {
                strategy: CategoryExtraItemsStrategy::KEEP,
            },
        };

        let config: CategoryConfigsList = serde_yaml::from_str(yaml_config).unwrap();

        assert_eq!(config, expected_config);
    }

    #[test]
    fn it_parses_empty_config_list_to_defaults() {
        let yaml_config = r"";

        let config: CategoryConfigsList = serde_yaml::from_str(yaml_config).unwrap();

        assert_eq!(config, CategoryConfigsList::default());
    }

    #[test]
    fn it_parses_empty_config_fields_to_defaults() {
        let yaml_config = r"
            items:
            - name: category_1
        ";
        let expected_config = CategoryConfigsList {
            items: vec![CategoryConfig {
                name: "category_1".to_string(),
                permissions_overwrites: vec![],
                extra_channels: ChannelExtraItemsConfig::default(),
            }],
            ..Default::default()
        };

        let config: CategoryConfigsList = serde_yaml::from_str(yaml_config).unwrap();

        assert_eq!(config, expected_config);
    }
}
