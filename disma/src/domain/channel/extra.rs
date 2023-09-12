use core::fmt::Debug;

use crate::{
    category::AwaitingCategory,
    channel::{AwaitingChannel, Channel, ExistingChannel},
    core::{changes::channel::ChannelChange, diffs::Differ},
};

pub trait ExtraChannelsStrategy {
    fn _type(&self) -> ExtraChannelsStrategyType;
    fn handle_extra_channel(
        &self,
        extra_existing: &ExistingChannel,
        changes: &mut Vec<ChannelChange>,
        awaiting_category: Option<&AwaitingCategory>,
    );
}

#[derive(Debug, PartialEq)]
pub enum ExtraChannelsStrategyType {
    Keep,
    Remove,
    OverwritePermissionsWithCategory,
}

impl Debug for dyn ExtraChannelsStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self._type())
    }
}

pub struct RemoveExtraChannels {}

impl ExtraChannelsStrategy for RemoveExtraChannels {
    fn _type(&self) -> ExtraChannelsStrategyType {
        ExtraChannelsStrategyType::Remove
    }

    fn handle_extra_channel(
        &self,
        extra_existing: &ExistingChannel,
        changes: &mut Vec<ChannelChange>,
        _awaiting_category: Option<&AwaitingCategory>,
    ) {
        changes.push(ChannelChange::Delete(extra_existing.clone()));
    }
}

pub struct KeepExtraChannels {}

impl ExtraChannelsStrategy for KeepExtraChannels {
    fn _type(&self) -> ExtraChannelsStrategyType {
        ExtraChannelsStrategyType::Keep
    }

    fn handle_extra_channel(
        &self,
        _extra_existing: &ExistingChannel,
        _changes: &mut Vec<ChannelChange>,
        _awaiting_category: Option<&AwaitingCategory>,
    ) {
    }
}

pub struct SyncExtraChannelsPermissions {}

impl ExtraChannelsStrategy for SyncExtraChannelsPermissions {
    fn _type(&self) -> ExtraChannelsStrategyType {
        ExtraChannelsStrategyType::OverwritePermissionsWithCategory
    }

    fn handle_extra_channel(
        &self,
        extra_existing: &ExistingChannel,
        changes: &mut Vec<ChannelChange>,
        awaiting_category: Option<&AwaitingCategory>,
    ) {
        if let Some(category) = awaiting_category {
            let awaiting_channel = AwaitingChannel {
                name: extra_existing.name().to_string(),
                topic: extra_existing.topic.clone(),
                channel_type: extra_existing.channel_type.clone(),
                category: Some(category.clone()),
                overwrites: category.overwrites.clone(),
            };

            changes.push(ChannelChange::Update(
                extra_existing.clone(),
                awaiting_channel.clone(),
                extra_existing.diffs_with(&awaiting_channel),
            ));
        } else {
            panic!("Category cannot be empty for overriding permissions overwrites of extra channel {}", extra_existing.name());
        }
    }
}
