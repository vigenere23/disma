use serde::{Deserialize, Serialize};

use crate::api::params::permission::PermissionsOverwriteParams;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct ChannelsParamsList {
    #[serde(default = "Vec::default")]
    pub items: Vec<ChannelParams>,
    #[serde(default = "ChannelParamsExtraItemsStrategy::default")]
    pub extra_items: ChannelParamsExtraItemsStrategy,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "strategy", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChannelParamsExtraItemsStrategy {
    Keep,
    Remove,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ChannelParams {
    pub name: String,
    #[serde(rename = "type", default = "ChannelParamsChannelType::default")]
    pub _type: ChannelParamsChannelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default = "ChannelParamsPermissionsOverwritesStrategy::default")]
    pub permissions_overwrites: ChannelParamsPermissionsOverwritesStrategy,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "strategy", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChannelParamsPermissionsOverwritesStrategy {
    FromCategory,
    Manual {
        items: Vec<PermissionsOverwriteParams>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ChannelParamsChannelType {
    TEXT,
    VOICE,
}

impl Default for ChannelParamsExtraItemsStrategy {
    fn default() -> Self {
        Self::Keep
    }
}

impl Default for ChannelParamsChannelType {
    fn default() -> Self {
        Self::TEXT
    }
}

impl Default for ChannelParamsPermissionsOverwritesStrategy {
    fn default() -> Self {
        Self::Manual { items: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::params::{
            channel::{
                ChannelParams, ChannelParamsChannelType, ChannelParamsExtraItemsStrategy,
                ChannelParamsPermissionsOverwritesStrategy, ChannelsParamsList,
            },
            permission::PermissionsOverwriteParams,
        },
        permission::Permission,
    };

    #[test]
    fn it_parses_params_list() {
        let yaml_params_list = r"
            items:
            - name: channel_1
              type: VOICE
              topic: A nice evening
              category: category_1
              permissions_overwrites:
                strategy: MANUAL
                items:
                - role: role_1
                  allow: [ADMINISTRATOR]
                  deny: [SEND_MESSAGES]
            extra_items:
              strategy: KEEP
        ";
        let expected_params_list = ChannelsParamsList {
            items: vec![ChannelParams {
                name: "channel_1".to_string(),
                _type: ChannelParamsChannelType::VOICE,
                topic: Some("A nice evening".to_string()),
                category: Some("category_1".to_string()),
                permissions_overwrites: ChannelParamsPermissionsOverwritesStrategy::Manual {
                    items: vec![PermissionsOverwriteParams {
                        role: "role_1".to_string(),
                        allow: vec![Permission::ADMINISTRATOR],
                        deny: vec![Permission::SEND_MESSAGES],
                    }],
                },
            }],
            extra_items: ChannelParamsExtraItemsStrategy::Keep,
        };

        let params_list: ChannelsParamsList = serde_yaml::from_str(yaml_params_list).unwrap();

        assert_eq!(params_list, expected_params_list);
    }

    #[test]
    fn it_parses_empty_params_list_to_defaults() {
        let yaml_params_list = r"";

        let params_list: ChannelsParamsList = serde_yaml::from_str(yaml_params_list).unwrap();

        assert_eq!(params_list, ChannelsParamsList::default());
    }

    #[test]
    fn it_parses_empty_params_fields_to_defaults() {
        let yaml_params_list = r"
            items:
            - name: channel_1
        ";
        let expected_params_list = ChannelsParamsList {
            items: vec![ChannelParams {
                name: "channel_1".to_string(),
                _type: ChannelParamsChannelType::TEXT,
                topic: None,
                category: None,
                permissions_overwrites: ChannelParamsPermissionsOverwritesStrategy::Manual {
                    items: vec![],
                },
            }],
            extra_items: ChannelParamsExtraItemsStrategy::Keep,
        };

        let params_list: ChannelsParamsList = serde_yaml::from_str(yaml_params_list).unwrap();

        assert_eq!(params_list, expected_params_list);
    }
}
