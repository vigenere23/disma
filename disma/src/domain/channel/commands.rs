use core::fmt::Debug;
use std::sync::Arc;

use crate::{
    category::{AwaitingCategory, CategoriesList, ExistingCategory},
    channel::{AwaitingChannel, Channel, ExistingChannel},
    commands::{Command, CommandDescription, CommandEntity, CommandRef},
    diff::{Diff, Differ},
    guild::GuildCommanderRef,
    role::{ExistingRole, RolesList},
};

#[deprecated = "Use core::commands::UpdateChannel command instead"]
pub struct UpdateChannel {
    existing_channel: ExistingChannel,
    awaiting_channel: AwaitingChannel,
    roles: RolesList<ExistingRole>,
    categories: CategoriesList<ExistingCategory>,
    diffs: Vec<Diff>,
}

impl UpdateChannel {
    pub fn try_new(
        existing_channel: ExistingChannel,
        awaiting_channel: AwaitingChannel,
        roles: RolesList<ExistingRole>,
        categories: CategoriesList<ExistingCategory>,
    ) -> Result<Self, String> {
        let diffs = existing_channel.diffs_with(&awaiting_channel);

        if diffs.is_empty() {
            return Err(format!(
                "No diffs between channels {} and {}",
                existing_channel.name, awaiting_channel.name
            ));
        };

        Ok(Self {
            existing_channel,
            awaiting_channel,
            roles,
            categories,
            diffs,
        })
    }
}

impl Command for UpdateChannel {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild
            .update_channel(
                &self.existing_channel.id,
                &self.awaiting_channel,
                &self.roles,
                &self.categories,
            )
            .unwrap();
    }

    fn describe(&self) -> CommandDescription {
        CommandDescription::Update(
            CommandEntity::Channel,
            self.existing_channel.unique_name().to_string(),
            self.diffs.clone(),
        )
    }
}

#[deprecated = "Use core::commands::DeleteChannel command instead"]
pub struct DeleteChannel {
    channel: ExistingChannel,
}

impl DeleteChannel {
    pub fn new(channel: ExistingChannel) -> Self {
        Self { channel }
    }
}

impl Command for DeleteChannel {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.delete_category(&self.channel.id).unwrap();
    }

    fn describe(&self) -> CommandDescription {
        CommandDescription::Delete(
            CommandEntity::Channel,
            self.channel.unique_name().to_string(),
        )
    }
}

pub trait ExtraChannelsStrategy {
    fn _type(&self) -> ExtraChannelsStrategyType;
    fn handle_extra_channel(
        &self,
        extra_channel: &ExistingChannel,
        commands: &mut Vec<CommandRef>,
        awaiting_category: Option<&AwaitingCategory>,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
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
        extra_channel: &ExistingChannel,
        commands: &mut Vec<CommandRef>,
        _awaiting_category: Option<&AwaitingCategory>,
        _roles: &RolesList<ExistingRole>,
        _categories: &CategoriesList<ExistingCategory>,
    ) {
        let command = DeleteChannel::new(extra_channel.clone());
        commands.push(Arc::from(command));
    }
}

pub struct KeepExtraChannels {}

impl ExtraChannelsStrategy for KeepExtraChannels {
    fn _type(&self) -> ExtraChannelsStrategyType {
        ExtraChannelsStrategyType::Keep
    }

    fn handle_extra_channel(
        &self,
        _extra_channel: &ExistingChannel,
        _commands: &mut Vec<CommandRef>,
        _awaiting_category: Option<&AwaitingCategory>,
        _roles: &RolesList<ExistingRole>,
        _categories: &CategoriesList<ExistingCategory>,
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
        extra_channel: &ExistingChannel,
        commands: &mut Vec<CommandRef>,
        awaiting_category: Option<&AwaitingCategory>,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
    ) {
        if let Some(category) = awaiting_category {
            let awaiting_channel = AwaitingChannel {
                name: extra_channel.name().to_string(),
                topic: extra_channel.topic.clone(),
                channel_type: extra_channel.channel_type.clone(),
                category: Some(category.clone()),
                overwrites: category.overwrites.clone(),
            };

            if let Ok(command) = UpdateChannel::try_new(
                extra_channel.clone(),
                awaiting_channel,
                roles.clone(),
                categories.clone(),
            ) {
                commands.push(Arc::from(command));
            }
        } else {
            panic!("Category cannot be empty for overriding permissions overwrites of extra channel {}", extra_channel.name());
        }
    }
}
