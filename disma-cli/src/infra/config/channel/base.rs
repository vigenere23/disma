use serde::{Deserialize, Serialize};

use crate::infra::config::permission::PermissionsOverwritesConfig;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ChannelConfigsList {
    #[serde(default = "Vec::default")]
    pub items: Vec<ChannelConfig>,
    #[serde(default = "ChannelExtraItemsConfig::default")]
    pub extra_items: ChannelExtraItemsConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChannelExtraItemsConfig {
    pub strategy: ChannelExtraItemsStrategy,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ChannelExtraItemsStrategy {
    KEEP,
    REMOVE,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChannelConfig {
    pub name: String,
    #[serde(rename = "type", default = "ChannelConfigType::default")]
    pub _type: ChannelConfigType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default = "Vec::default")]
    pub permissions_overwrites: Vec<PermissionsOverwritesConfig>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ChannelConfigType {
    TEXT,
    VOICE,
}

impl Default for ChannelExtraItemsConfig {
    fn default() -> Self {
        Self {
            strategy: ChannelExtraItemsStrategy::REMOVE,
        }
    }
}

impl Default for ChannelConfigType {
    fn default() -> Self {
        Self::TEXT
    }
}

#[cfg(test)]
mod tests {
    use disma::permission::Permission;

    use crate::infra::config::{
        channel::{
            ChannelConfig, ChannelConfigType, ChannelConfigsList, ChannelExtraItemsConfig,
            ChannelExtraItemsStrategy,
        },
        permission::PermissionsOverwritesConfig,
    };

    #[test]
    fn it_parses_config_list() {
        let yaml_config = r"
            items:
            - name: channel_1
              type: VOICE
              topic: A nice evening
              category: category_1
              permissions_overwrites:
              - role: role_1
                allow: [ADMINISTRATOR]
                deny: [SEND_MESSAGES]
            extra_items:
              strategy: KEEP
        ";
        let expected_config = ChannelConfigsList {
            items: vec![ChannelConfig {
                name: "channel_1".to_string(),
                _type: ChannelConfigType::VOICE,
                topic: Some("A nice evening".to_string()),
                category: Some("category_1".to_string()),
                permissions_overwrites: vec![PermissionsOverwritesConfig {
                    role: "role_1".to_string(),
                    allow: vec![Permission::ADMINISTRATOR],
                    deny: vec![Permission::SEND_MESSAGES],
                }],
            }],
            extra_items: ChannelExtraItemsConfig {
                strategy: ChannelExtraItemsStrategy::KEEP,
            },
        };

        let config: ChannelConfigsList = serde_yaml::from_str(yaml_config).unwrap();

        assert_eq!(config, expected_config);
    }

    #[test]
    fn it_parses_empty_config_list_to_defaults() {
        let yaml_config = r"";

        let config: ChannelConfigsList = serde_yaml::from_str(yaml_config).unwrap();

        assert_eq!(config, ChannelConfigsList::default());
    }

    #[test]
    fn it_parses_empty_config_fields_to_defaults() {
        let yaml_config = r"
            items:
            - name: channel_1
        ";
        let expected_config = ChannelConfigsList {
            items: vec![ChannelConfig {
                name: "channel_1".to_string(),
                _type: ChannelConfigType::TEXT,
                topic: None,
                category: None,
                permissions_overwrites: vec![],
            }],
            extra_items: ChannelExtraItemsConfig {
                strategy: ChannelExtraItemsStrategy::REMOVE,
            },
        };

        let config: ChannelConfigsList = serde_yaml::from_str(yaml_config).unwrap();

        assert_eq!(config, expected_config);
    }
}
