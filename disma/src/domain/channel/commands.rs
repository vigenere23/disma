use core::fmt::Debug;
use std::sync::Arc;

use crate::{
    category::{AwaitingCategory, CategoriesList, Category, ExistingCategory},
    channel::{AwaitingChannel, Channel, ExistingChannel},
    commands::{Command, CommandDescription, CommandEntity, CommandFactory, CommandRef},
    diff::Differ,
    guild::{ExistingGuild, GuildCommanderRef},
    role::{ExistingRole, RolesList},
};

use super::AwaitingChannelsList;

impl CommandFactory for AwaitingChannelsList {
    fn commands_for(&self, existing_guild: &ExistingGuild) -> Vec<CommandRef> {
        let mut commands: Vec<CommandRef> = Vec::new();

        for awaiting_channel in self.items.to_list() {
            let category_name = awaiting_channel
                .category
                .as_ref()
                .map(|category| category.name());

            match existing_guild.channels.find(
                &awaiting_channel.name,
                awaiting_channel.channel_type(),
                category_name,
            ) {
                Some(existing_channel) => {
                    if existing_channel != awaiting_channel {
                        let command = UpdateChannel::new(
                            existing_channel.clone(),
                            awaiting_channel.clone(),
                            existing_guild.roles.clone(),
                            existing_guild.categories.clone(),
                        );
                        commands.push(Arc::from(command));
                    }
                }
                None => {
                    let command = AddChannel::new(
                        awaiting_channel.clone(),
                        existing_guild.roles.clone(),
                        existing_guild.categories.clone(),
                    );
                    commands.push(Arc::from(command));
                }
            }
        }

        for existing_channel in existing_guild.channels.to_list() {
            let category_name = existing_channel
                .category
                .as_ref()
                .map(|category| category.name());

            let matching_awaiting_channel = self.items.find(
                &existing_channel.name,
                existing_channel.channel_type(),
                category_name,
            );

            let matching_awaiting_category = existing_channel
                .category_name()
                .map(|category_name| self.categories.find_by_name(&category_name))
                .unwrap_or_default();

            let extra_items_strategy = match matching_awaiting_category {
                Some(category) => category.extra_channels_strategy.clone(),
                None => self.extra_items_strategy.clone(),
            };

            if matching_awaiting_channel.is_none() {
                extra_items_strategy.handle_extra_channel(
                    existing_channel,
                    &mut commands,
                    matching_awaiting_category,
                    &existing_guild.roles,
                    &existing_guild.categories,
                );
            }
        }

        commands
    }
}

pub struct AddChannel {
    channel: AwaitingChannel,
    roles: RolesList<ExistingRole>,
    categories: CategoriesList<ExistingCategory>,
}

impl AddChannel {
    pub fn new(
        channel: AwaitingChannel,
        roles: RolesList<ExistingRole>,
        categories: CategoriesList<ExistingCategory>,
    ) -> Self {
        Self {
            channel,
            roles,
            categories,
        }
    }
}

impl Command for AddChannel {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.add_channel(&self.channel, &self.roles, &self.categories);
    }

    fn describe(&self) -> CommandDescription {
        CommandDescription::Create(CommandEntity::Channel, self.channel.unique_name())
    }
}

pub struct UpdateChannel {
    existing_channel: ExistingChannel,
    awaiting_channel: AwaitingChannel,
    roles: RolesList<ExistingRole>,
    categories: CategoriesList<ExistingCategory>,
}

impl UpdateChannel {
    pub fn new(
        existing_channel: ExistingChannel,
        awaiting_channel: AwaitingChannel,
        roles: RolesList<ExistingRole>,
        categories: CategoriesList<ExistingCategory>,
    ) -> Self {
        Self {
            existing_channel,
            awaiting_channel,
            roles,
            categories,
        }
    }
}

impl Command for UpdateChannel {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.update_channel(
            &self.existing_channel.id,
            &self.awaiting_channel,
            &self.roles,
            &self.categories,
        );
    }

    fn describe(&self) -> CommandDescription {
        CommandDescription::Update(
            CommandEntity::Channel,
            self.existing_channel.unique_name(),
            self.existing_channel.diffs_with(&self.awaiting_channel),
        )
    }
}

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
        guild.delete_category(&self.channel.id);
    }

    fn describe(&self) -> CommandDescription {
        CommandDescription::Delete(CommandEntity::Channel, self.channel.unique_name())
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

pub struct OverwriteExtraChannelsPermissionsWithCategory {}

impl ExtraChannelsStrategy for OverwriteExtraChannelsPermissionsWithCategory {
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
                name: extra_channel.name(),
                topic: extra_channel.topic.clone(),
                channel_type: extra_channel.channel_type(),
                category: Some(category.clone()),
                overwrites: category.overwrites.clone(),
            };
            let command = UpdateChannel::new(
                extra_channel.clone(),
                awaiting_channel,
                roles.clone(),
                categories.clone(),
            );
            commands.push(Arc::from(command));
        } else {
            panic!("Category cannot be empty for overriding permissions overwrites of extra channel {}", extra_channel.name());
        }
    }
}
