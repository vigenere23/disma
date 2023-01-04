use crate::{
    category::{AwaitingCategory, Category},
    overwrites::PermissionsOverwritesList,
    role::AwaitingRole,
};

use super::{Channel, ChannelType, ChannelsList};

#[derive(Debug, PartialEq, Clone)]
pub struct AwaitingChannelsList {
    pub items: ChannelsList<AwaitingChannel>,
    pub extra_items: ExtraChannelsOptions,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExtraChannelsOptions {
    pub strategy: ExtraChannelsStrategy,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExtraChannelsStrategy {
    Keep,
    Remove,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AwaitingChannel {
    pub name: String,
    pub topic: Option<String>,
    pub channel_type: ChannelType,
    pub category: Option<AwaitingCategory>,
    pub overwrites: PermissionsOverwritesList<AwaitingRole>,
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
