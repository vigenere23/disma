use disma::{
    category::{AwaitingCategory, CategoriesList},
    channel::{AwaitingChannel, ChannelType, ExistingChannel},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ChannelTypeConfig {
    Text,
    Voice,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub _type: Option<ChannelTypeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl From<&ExistingChannel> for ChannelConfig {
    fn from(channel: &ExistingChannel) -> Self {
        let _type = match channel.channel_type {
            ChannelType::Text => Some(ChannelTypeConfig::Text),
            ChannelType::Voice => Some(ChannelTypeConfig::Voice),
        };

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
        let channel_type = match self._type {
            Some(_type) => match _type {
                ChannelTypeConfig::Text => ChannelType::Text,
                ChannelTypeConfig::Voice => ChannelType::Voice,
            },
            None => ChannelType::Text,
        };

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
