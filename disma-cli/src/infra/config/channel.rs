use std::sync::Arc;

use disma::{
    category::{AwaitingCategory, CategoriesList},
    channel::{
        AwaitingChannel, AwaitingChannelsList, ChannelType, ExistingChannel, ExtraChannelsStrategy,
        KeepExtraChannels, RemoveExtraChannels,
    },
    permission::PermissionsOverwrites,
    role::{AwaitingRole, RolesList},
};
use serde::{Deserialize, Serialize};

use super::permission::PermissionsOverwritesConfig;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ChannelConfigsList {
    #[serde(default = "Vec::default")]
    pub items: Vec<ChannelConfig>,
    #[serde(default = "ChannelExtraItemsConfig::default")]
    pub extra_items: ChannelExtraItemsConfig,
}

impl ChannelConfigsList {
    pub fn into(
        self,
        roles: &RolesList<AwaitingRole>,
        categories: &CategoriesList<AwaitingCategory>,
    ) -> AwaitingChannelsList {
        let items = self
            .items
            .into_iter()
            .map(|channel| channel.into(roles, categories))
            .collect::<Vec<AwaitingChannel>>()
            .into();

        AwaitingChannelsList {
            items,
            extra_items_strategy: self.extra_items.strategy.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChannelExtraItemsConfig {
    pub strategy: ChannelExtraItemsStrategy,
}

impl Default for ChannelExtraItemsConfig {
    fn default() -> Self {
        Self {
            strategy: ChannelExtraItemsStrategy::REMOVE,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ChannelExtraItemsStrategy {
    KEEP,
    REMOVE,
    // TODO Overwrite,
}

impl Into<Arc<dyn ExtraChannelsStrategy>> for ChannelExtraItemsStrategy {
    fn into(self) -> Arc<dyn ExtraChannelsStrategy> {
        match self {
            Self::KEEP => Arc::from(KeepExtraChannels {}),
            Self::REMOVE => Arc::from(RemoveExtraChannels {}),
        }
    }
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

impl Default for ChannelConfigType {
    fn default() -> Self {
        Self::TEXT
    }
}

impl From<ChannelType> for ChannelConfigType {
    fn from(value: ChannelType) -> Self {
        match value {
            ChannelType::TEXT => Self::TEXT,
            ChannelType::VOICE => Self::VOICE,
        }
    }
}

impl Into<ChannelType> for ChannelConfigType {
    fn into(self) -> ChannelType {
        match self {
            Self::TEXT => ChannelType::TEXT,
            Self::VOICE => ChannelType::VOICE,
        }
    }
}

impl From<&ExistingChannel> for ChannelConfig {
    fn from(channel: &ExistingChannel) -> Self {
        let _type = channel.channel_type.clone().into();

        let category = channel
            .category
            .as_ref()
            .map(|category| category.name.clone());

        let permissions_overwrites = channel
            .overwrites
            .to_list()
            .iter()
            .map(PermissionsOverwritesConfig::from)
            .collect::<Vec<PermissionsOverwritesConfig>>();

        Self {
            name: channel.name.clone(),
            topic: channel.topic.clone(),
            _type,
            category,
            permissions_overwrites,
        }
    }
}

impl ChannelConfig {
    pub fn into(
        self,
        roles: &RolesList<AwaitingRole>,
        categories: &CategoriesList<AwaitingCategory>,
    ) -> AwaitingChannel {
        let channel_type = self._type.into();

        let category = self.category.map(|name| {
            categories
                .find_by_name(&name)
                .unwrap_or_else(|| panic!("No category found for name {name}."))
                .clone()
        });

        // TODO add strategy to inherit category's overwrites?
        let overwrites = self
            .permissions_overwrites
            .into_iter()
            .map(|permission| permission.into(roles))
            .collect::<Vec<PermissionsOverwrites<AwaitingRole>>>();

        AwaitingChannel {
            name: self.name,
            topic: self.topic,
            channel_type,
            category,
            overwrites: overwrites.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    mod into_awaiting {
        use disma::{
            category::{AwaitingCategory, CategoriesList},
            channel::{AwaitingChannel, ChannelType},
            permission::{
                Permission, PermissionsList, PermissionsOverwrites, PermissionsOverwritesList,
            },
            role::{AwaitingRole, RolesList},
        };

        use crate::infra::config::{
            channel::{ChannelConfig, ChannelConfigType, ChannelExtraItemsConfig},
            permission::PermissionsOverwritesConfig,
        };

        fn given_awaiting_category(name: &str) -> AwaitingCategory {
            AwaitingCategory {
                name: name.to_string(),
                overwrites: PermissionsOverwritesList::from(vec![]),
                extra_channels_strategy: ChannelExtraItemsConfig::default().strategy.into(),
            }
        }

        fn given_awaiting_categories(names: Vec<&str>) -> CategoriesList<AwaitingCategory> {
            let categories: Vec<AwaitingCategory> = names
                .iter()
                .map(|name| given_awaiting_category(name))
                .collect();
            CategoriesList::from(categories)
        }

        fn given_awaiting_role(name: &str) -> AwaitingRole {
            AwaitingRole {
                name: name.to_string(),
                permissions: PermissionsList::from(vec![Permission::VIEW_CHANNEL]),
                color: Some("123456".to_string()),
                is_mentionable: true,
                show_in_sidebar: false,
            }
        }

        #[test]
        fn can_convert_config_to_awaiting_entity() {
            let channel_name = "general".to_string();
            let category_name = "team-01".to_string();
            let topic = "A new era for the modern age".to_string();
            let role_name = "Team01".to_string();
            let categories = given_awaiting_categories(vec![&category_name]);
            let category = given_awaiting_category(&category_name);
            let role = given_awaiting_role(&role_name);

            let config = ChannelConfig {
                name: channel_name.clone(),
                _type: ChannelConfigType::VOICE,
                category: Some(category_name.clone()),
                topic: Some(topic.clone()),
                permissions_overwrites: vec![PermissionsOverwritesConfig {
                    role: role_name.clone(),
                    allow: Some(vec!["ADMINISTRATOR".to_string()]),
                    deny: Some(vec!["SEND_MESSAGES".to_string()]),
                }],
            };

            let entity = config.into(&RolesList::from(vec![role.clone()]), &categories);

            let expected_entity = AwaitingChannel {
                name: channel_name.clone(),
                category: Some(category),
                channel_type: ChannelType::VOICE,
                topic: Some(topic.clone()),
                overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrites {
                    role: role.clone(),
                    allow: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                    deny: PermissionsList::from(vec![Permission::SEND_MESSAGES]),
                }]),
            };
            assert_eq!(entity, expected_entity);
        }

        #[test]
        fn can_convert_compressed_config_to_awaiting_entity() {
            let channel_name = "general".to_string();

            let config = ChannelConfig {
                name: channel_name.clone(),
                _type: ChannelConfigType::TEXT,
                category: None,
                topic: None,
                permissions_overwrites: vec![],
            };

            let entity = config.into(&RolesList::from(vec![]), &CategoriesList::from(vec![]));

            let expected_entity = AwaitingChannel {
                name: channel_name.clone(),
                category: None,
                channel_type: ChannelType::TEXT,
                topic: None,
                overwrites: PermissionsOverwritesList::from(vec![]),
            };
            assert_eq!(entity, expected_entity);
        }
    }

    mod from_existing {
        use disma::{
            category::ExistingCategory,
            channel::{ChannelType, ExistingChannel},
            permission::{
                Permission, PermissionsList, PermissionsOverwrites, PermissionsOverwritesList,
            },
            role::ExistingRole,
        };

        use crate::infra::config::{
            channel::{ChannelConfig, ChannelConfigType},
            permission::PermissionsOverwritesConfig,
        };

        fn given_existing_category(id: &str, name: &str) -> ExistingCategory {
            ExistingCategory {
                id: id.to_string(),
                name: name.to_string(),
                overwrites: PermissionsOverwritesList::from(vec![]),
            }
        }

        fn given_existing_role(name: &str) -> ExistingRole {
            ExistingRole {
                id: "bob".to_string(),
                name: name.to_string(),
                permissions: PermissionsList::from(vec![Permission::VIEW_CHANNEL]),
                color: Some("123456".to_string()),
                is_mentionable: true,
                show_in_sidebar: false,
            }
        }

        #[test]
        fn can_convert_existing_entity_to_config() {
            let channel_id = "123asd".to_string();
            let channel_name = "general".to_string();
            let category_id = "987kdj".to_string();
            let category_name = "team-01".to_string();
            let topic = "A new era for the modern age".to_string();
            let role_name = "Heam12".to_string();
            let role = given_existing_role(&role_name);
            let category = given_existing_category(&category_id, &category_name);

            let entity = ExistingChannel {
                id: channel_id,
                name: channel_name.clone(),
                category: Some(category),
                channel_type: ChannelType::VOICE,
                topic: Some(topic.clone()),
                overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrites {
                    role: role.clone(),
                    allow: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                    deny: PermissionsList::from(vec![Permission::SEND_MESSAGES]),
                }]),
            };

            let config = ChannelConfig::from(&entity);

            let expected_config = ChannelConfig {
                name: channel_name.clone(),
                _type: ChannelConfigType::VOICE,
                category: Some(category_name.clone()),
                topic: Some(topic.clone()),
                permissions_overwrites: vec![PermissionsOverwritesConfig {
                    role: role_name.clone(),
                    allow: Some(vec!["ADMINISTRATOR".to_string()]),
                    deny: Some(vec!["SEND_MESSAGES".to_string()]),
                }],
            };
            assert_eq!(config, expected_config);
        }

        #[test]
        fn can_convert_existing_entity_to_compressed_config() {
            let channel_id = "123asd".to_string();
            let channel_name = "general".to_string();

            let entity = ExistingChannel {
                id: channel_id,
                name: channel_name.clone(),
                category: None,
                channel_type: ChannelType::VOICE,
                topic: None,
                overwrites: PermissionsOverwritesList::from(vec![]),
            };

            let config = ChannelConfig::from(&entity);

            let expected_config = ChannelConfig {
                name: channel_name.clone(),
                _type: ChannelConfigType::VOICE,
                category: None,
                topic: None,
                permissions_overwrites: vec![],
            };
            assert_eq!(config, expected_config);
        }
    }

    mod serde_parsing {}
}
