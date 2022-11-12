use std::str::FromStr;

use disma::{
    category::{AwaitingCategory, CategoriesList},
    channel::{AwaitingChannel, ChannelType, ExistingChannel},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChannelConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl From<&ExistingChannel> for ChannelConfig {
    fn from(channel: &ExistingChannel) -> Self {
        let _type = Some(channel.channel_type.to_string());

        let category = channel
            .category
            .as_ref()
            .map(|category| category.name.clone());

        Self {
            name: channel.name.clone(),
            topic: channel.topic.clone(),
            _type,
            category,
        }
    }
}

impl ChannelConfig {
    pub fn into(self, categories: &CategoriesList<AwaitingCategory>) -> AwaitingChannel {
        let channel_type = self
            ._type
            .as_ref()
            .map(|_type| ChannelType::from_str(_type).unwrap())
            .unwrap_or(ChannelType::TEXT);

        let category = self.category.map(|name| {
            categories
                .find_by_name(&name)
                .unwrap_or_else(|| panic!("No category found for name {name}."))
                .clone()
        });

        AwaitingChannel {
            name: self.name,
            topic: self.topic,
            channel_type,
            category,
        }
    }
}

#[cfg(test)]
mod tests {
    use disma::{
        category::{AwaitingCategory, CategoriesList, ExistingCategory},
        channel::{AwaitingChannel, ChannelType, ExistingChannel},
        overwrites::PermissionsOverwritesList,
    };

    use super::ChannelConfig;

    fn given_awaiting_category(name: &str) -> AwaitingCategory {
        AwaitingCategory {
            name: name.to_string(),
            overwrites: PermissionsOverwritesList::from(vec![]),
        }
    }

    fn given_awaiting_categories(names: Vec<&str>) -> CategoriesList<AwaitingCategory> {
        let categories: Vec<AwaitingCategory> = names
            .iter()
            .map(|name| given_awaiting_category(name))
            .collect();
        CategoriesList::from(categories)
    }

    fn given_existing_category(id: &str, name: &str) -> ExistingCategory {
        ExistingCategory {
            id: id.to_string(),
            name: name.to_string(),
            overwrites: PermissionsOverwritesList::from(vec![]),
        }
    }

    #[test]
    fn can_convert_config_to_awaiting_entity() {
        let channel_name = "general".to_string();
        let category_name = "team-01".to_string();
        let topic = "A new era for the modern age".to_string();
        let categories = given_awaiting_categories(vec![&category_name]);
        let category = given_awaiting_category(&category_name);

        let config = ChannelConfig {
            name: channel_name.clone(),
            _type: Some("VOICE".into()),
            category: Some(category_name.clone()),
            topic: Some(topic.clone()),
        };

        let entity = config.into(&categories);

        let expected_entity = AwaitingChannel {
            name: channel_name.clone(),
            category: Some(category),
            channel_type: ChannelType::VOICE,
            topic: Some(topic.clone()),
        };
        assert_eq!(entity, expected_entity);
    }

    #[test]
    fn can_convert_config_to_awaiting_entity_with_optionals() {
        let channel_name = "general".to_string();
        let categories = given_awaiting_categories(vec![]);

        let config = ChannelConfig {
            name: channel_name.clone(),
            _type: None,
            category: None,
            topic: None,
        };

        let entity = config.into(&categories);

        let expected_entity = AwaitingChannel {
            name: channel_name.clone(),
            category: None,
            channel_type: ChannelType::TEXT,
            topic: None,
        };
        assert_eq!(entity, expected_entity);
    }

    #[test]
    fn can_convert_existing_entity_to_config() {
        let channel_id = "123asd".to_string();
        let channel_name = "general".to_string();
        let category_id = "987kdj".to_string();
        let category_name = "team-01".to_string();
        let topic = "A new era for the modern age".to_string();
        let category = given_existing_category(&category_id, &category_name);

        let entity = ExistingChannel {
            id: channel_id,
            name: channel_name.clone(),
            category: Some(category),
            channel_type: ChannelType::VOICE,
            topic: Some(topic.clone()),
        };

        let config = ChannelConfig::from(&entity);

        let expected_config = ChannelConfig {
            name: channel_name.clone(),
            _type: Some("VOICE".into()),
            category: Some(category_name.clone()),
            topic: Some(topic.clone()),
        };
        assert_eq!(config, expected_config);
    }

    #[test]
    fn can_convert_existing_entity_to_config_with_optionals() {
        let channel_id = "123asd".to_string();
        let channel_name = "general".to_string();

        let entity = ExistingChannel {
            id: channel_id,
            name: channel_name.clone(),
            category: None,
            channel_type: ChannelType::VOICE,
            topic: None,
        };

        let config = ChannelConfig::from(&entity);

        let expected_config = ChannelConfig {
            name: channel_name.clone(),
            _type: Some("VOICE".into()),
            category: None,
            topic: None,
        };
        assert_eq!(config, expected_config);
    }
}
