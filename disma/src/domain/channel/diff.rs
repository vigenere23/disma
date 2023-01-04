use crate::{
    diff::{Diff, Differ},
    utils::{misc::IfThen, option::OptionEq},
};

use super::{AwaitingChannel, ExistingChannel};

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
