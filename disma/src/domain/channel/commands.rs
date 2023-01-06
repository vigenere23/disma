use core::fmt::Debug;
use std::sync::Arc;

use crate::{
    category::{CategoriesList, Category, ExistingCategory},
    channel::{AwaitingChannel, Channel, ExistingChannel},
    commands::{Command, CommandDescription, CommandEntity, CommandFactory, CommandRef},
    diff::Differ,
    guild::{ExistingGuild, GuildCommanderRef},
    role::{ExistingRole, RolesList},
};

use super::{AwaitingChannelsList, ChannelsList};

impl CommandFactory for AwaitingChannelsList {
    fn commands_for(&self, existing_guild: &ExistingGuild) -> Vec<CommandRef> {
        let mut diffs: Vec<CommandRef> = Vec::new();

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
                        diffs.push(Arc::from(command));
                    }
                }
                None => {
                    let command = AddChannel::new(
                        awaiting_channel.clone(),
                        existing_guild.roles.clone(),
                        existing_guild.categories.clone(),
                    );
                    diffs.push(Arc::from(command));
                }
            }
        }

        // TODO handle extra items

        // for existing_channel in existing_guild.channels.to_list() {
        //     let category_name = existing_channel
        //         .category
        //         .as_ref()
        //         .map(|category| category.name());

        //     let matching_awaiting_channel = self.items.find(
        //         &existing_channel.name,
        //         existing_channel.channel_type(),
        //         category_name,
        //     );

        //     let should_remove_channel_default =
        //         self.extra_items.strategy == ExtraChannelsStrategy::Remove;

        //     // TODO should be calculated at creation, not here
        //     let should_remove_channel = match matching_awaiting_channel {
        //         Some(channel) => match &channel.category {
        //             Some(category) => {
        //                 category.extra_channels.strategy == ExtraChannelsStrategy::Remove
        //             }
        //             None => should_remove_channel_default,
        //         },
        //         None => should_remove_channel_default,
        //     };

        //     if should_remove_channel {
        //         let command = DeleteChannel::new(existing_channel.clone());
        //         diffs.push(Arc::from(command));
        //     }
        // }

        diffs
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
    fn handle_extra_roles(
        &self,
        awaiting_channels: &ChannelsList<AwaitingChannel>,
        existing_channels: &ChannelsList<ExistingChannel>,
        commands: &mut Vec<CommandRef>,
    );
}

#[derive(Debug, PartialEq)]
pub enum ExtraChannelsStrategyType {
    Keep,
    Remove,
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

    fn handle_extra_roles(
        &self,
        _awaiting_channels: &ChannelsList<AwaitingChannel>,
        _existing_channels: &ChannelsList<ExistingChannel>,
        _commands: &mut Vec<CommandRef>,
    ) {
        todo!()
    }
}

pub struct KeepExtraChannels {}

impl ExtraChannelsStrategy for KeepExtraChannels {
    fn _type(&self) -> ExtraChannelsStrategyType {
        ExtraChannelsStrategyType::Keep
    }

    fn handle_extra_roles(
        &self,
        _awaiting_channels: &ChannelsList<AwaitingChannel>,
        _existing_channels: &ChannelsList<ExistingChannel>,
        _commands: &mut Vec<CommandRef>,
    ) {
    }
}
