use std::sync::Arc;

use crate::{
    category::{AwaitingCategory, CategoriesList, Category},
    permission::PermissionsOverwritesList,
    role::AwaitingRole,
};

use super::{Channel, ChannelType, ChannelsList, ExtraChannelsStrategy};

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
    pub overwrites: Option<PermissionsOverwritesList<AwaitingRole>>,
}

impl Channel for AwaitingChannel {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn category_name(&self) -> Option<String> {
        self.category.as_ref().map(|category| category.name())
    }

    fn channel_type(&self) -> ChannelType {
        self.channel_type.clone()
    }
}
