use std::sync::Arc;

use crate::{
    category::{AwaitingCategory, CategoriesList, Category},
    permission::PermissionsOverwritesList,
};

use super::{Channel, ChannelType, ChannelsList, ExtraChannelsStrategy, UniqueChannelName};

#[derive(Debug, Clone)]
pub struct AwaitingChannelsList {
    pub items: ChannelsList<AwaitingChannel>,
    pub extra_items_strategy: Arc<dyn ExtraChannelsStrategy>,
    pub categories: CategoriesList<AwaitingCategory>,
}

impl PartialEq for AwaitingChannelsList {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
            && self.extra_items_strategy._type() == other.extra_items_strategy._type()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AwaitingChannel {
    pub name: String,
    pub topic: Option<String>,
    pub channel_type: ChannelType,
    pub category: Option<AwaitingCategory>,
    pub overwrites: PermissionsOverwritesList,
}

impl AwaitingChannel {
    pub fn category_name(&self) -> Option<&str> {
        self.category.as_ref().map(|category| category.name())
    }
}

impl Channel for AwaitingChannel {
    fn name(&self) -> &str {
        &self.name
    }

    fn unique_name(&self) -> UniqueChannelName {
        UniqueChannelName::from(&self.name, &self.channel_type, self.category_name())
    }
}
