use std::sync::Arc;

use crate::{
    category::Category,
    diff::{
        base::{Diff, DiffCommandFactory, DiffCommandRef, Differ},
        channel::{AddChannel, DeleteChannel, UpdateChannel},
    },
    utils::{misc::IfThen, option::OptionEq},
};

use super::{
    AwaitingChannel, AwaitingChannelsList, Channel, ExistingChannel, ExtraChannelsStrategy,
};

impl PartialEq<AwaitingChannel> for ExistingChannel {
    fn eq(&self, other: &AwaitingChannel) -> bool {
        self.name == other.name
            && self.topic == other.topic
            && self.channel_type == other.channel_type
            && self.category.option_eq(&other.category)
            && self.overwrites == other.overwrites
    }
}

impl Differ<AwaitingChannel> for ExistingChannel {
    fn diffs_with(&self, awaiting: &AwaitingChannel) -> Vec<Diff> {
        let mut all_diffs = vec![];

        self.topic.diffs_with(&awaiting.topic).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("topic".into(), diffs)),
        );

        self.channel_type
            .diffs_with(&awaiting.channel_type)
            .if_then(
                |diffs| !diffs.is_empty(),
                |diffs| all_diffs.push(Diff::Update("channel_type".into(), diffs)),
            );

        self.category.diffs_with(&awaiting.category).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("category".into(), diffs)),
        );

        self.overwrites.diffs_with(&awaiting.overwrites).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("overwrites".into(), diffs)),
        );

        all_diffs
    }
}

impl DiffCommandFactory for AwaitingChannelsList {
    fn diff_commands_for(
        &self,
        existing_guild: &crate::guild::ExistingGuild,
    ) -> Vec<crate::diff::base::DiffCommandRef> {
        let mut diffs: Vec<DiffCommandRef> = Vec::new();

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

            let should_remove_channel_default =
                self.extra_items.strategy == ExtraChannelsStrategy::Remove;

            let should_remove_channel = match matching_awaiting_channel {
                Some(channel) => match &channel.category {
                    Some(category) => category.allow_extra_channels,
                    None => should_remove_channel_default,
                },
                None => should_remove_channel_default,
            };

            if should_remove_channel {
                let command = DeleteChannel::new(existing_channel.clone());
                diffs.push(Arc::from(command));
            }
        }

        diffs
    }
}
