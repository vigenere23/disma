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

            let diffs = extra_existing.diffs_with(&awaiting_channel);
            if diffs.is_empty() {
                return;
            };

            changes.push(ChannelChange::Update(
                extra_existing.clone(),
                awaiting_channel.clone(),
                diffs,
            ));
        } else {
            panic!("Category cannot be empty for overriding permissions overwrites of extra channel {}", extra_existing.name());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::diffs::Diff,
        permission::{PermissionsList, PermissionsOverwrite},
        tests::fixtures::{
            awaiting::{AwaitingCategoryFixture, AwaitingChannelFixture, AwaitingRoleFixture},
            existing::{ExistingCategoryFixture, ExistingChannelFixture, ExistingRoleFixture},
        },
    };

    use super::*;

    const A_CATEGORY_NAME: &str = "category abc";
    const A_CHANNEL_NAME: &str = "channel abc";
    const A_ROLE_NAME: &str = "role abc";

    #[test]
    fn when_keeping_extra_channels_should_not_add_changes() {
        let mut changes: Vec<ChannelChange> = Vec::new();
        let extra_channel = ExistingChannelFixture::new().build();

        let strategy = KeepExtraChannels {};
        strategy.handle_extra_channel(&extra_channel, &mut changes, None);

        assert!(changes.is_empty());
    }

    #[test]
    fn when_removing_extra_channels_should_add_delete_change() {
        let mut changes: Vec<ChannelChange> = Vec::new();
        let extra_channel = ExistingChannelFixture::new().build();

        let strategy = RemoveExtraChannels {};
        strategy.handle_extra_channel(&extra_channel, &mut changes, None);

        assert!(!changes.is_empty());
        assert_eq!(changes, vec![ChannelChange::Delete(extra_channel)]);
    }

    #[test]
    #[should_panic]
    fn given_no_category_when_syncing_extra_channels_permissions_should_panic() {
        let mut changes: Vec<ChannelChange> = Vec::new();
        let extra_channel = ExistingChannelFixture::new().build();

        let strategy = SyncExtraChannelsPermissions {};
        strategy.handle_extra_channel(&extra_channel, &mut changes, None);
    }

    #[test]
    fn given_category_with_same_permissions_when_syncing_extra_channels_permissions_should_not_add_changes(
    ) {
        let mut changes: Vec<ChannelChange> = Vec::new();
        let awaiting_category = AwaitingCategoryFixture::new()
            .with_name(A_CATEGORY_NAME)
            .build();
        let existing_category = ExistingCategoryFixture::new()
            .with_name(A_CATEGORY_NAME)
            .build();
        let extra_channel = ExistingChannelFixture::new()
            .with_category(&existing_category)
            .build();

        let strategy = SyncExtraChannelsPermissions {};
        strategy.handle_extra_channel(&extra_channel, &mut changes, Some(&awaiting_category));

        assert_eq!(changes, vec![]);
    }

    #[test]
    fn given_category_with_different_permissions_when_syncing_extra_channels_permissions_should_add_update_changes_for_permissions(
    ) {
        let mut changes: Vec<ChannelChange> = Vec::new();

        let awaiting_overwrites = vec![PermissionsOverwrite {
            role: AwaitingRoleFixture::new().with_name(A_ROLE_NAME).build(),
            allow: PermissionsList::new(),
            deny: PermissionsList::new(),
        }];
        let existing_overwrites = vec![PermissionsOverwrite {
            role: ExistingRoleFixture::new().with_name(A_ROLE_NAME).build(),
            allow: PermissionsList::new(),
            deny: PermissionsList::new(),
        }];
        let awaiting_category = AwaitingCategoryFixture::new()
            .with_name(A_CATEGORY_NAME)
            .with_permissions_overwrites(awaiting_overwrites.clone())
            .build();
        let existing_category = ExistingCategoryFixture::new()
            .with_name(A_CATEGORY_NAME)
            .with_permissions_overwrites(existing_overwrites)
            .build();
        let existing_channel = ExistingChannelFixture::new()
            .with_category(&existing_category)
            .with_name(A_CHANNEL_NAME)
            .build();
        let expected_awaiting_channel = AwaitingChannelFixture::new()
            .with_category(&awaiting_category)
            .with_name(A_CHANNEL_NAME)
            .with_permissions_overwrites(awaiting_overwrites)
            .build();

        let strategy = SyncExtraChannelsPermissions {};
        strategy.handle_extra_channel(&existing_channel, &mut changes, Some(&awaiting_category));

        assert!(!changes.is_empty());
        assert_eq!(
            changes,
            vec![ChannelChange::Update(
                existing_channel,
                expected_awaiting_channel,
                vec![Diff::Update(
                    "overwrites".to_string(),
                    vec![Diff::Add(A_ROLE_NAME.to_string())]
                )]
            )]
        );
    }
}
