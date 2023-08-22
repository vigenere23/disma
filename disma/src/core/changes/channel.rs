use crate::{
    channel::{AwaitingChannel, ExistingChannel},
    core::{
        diffs::{Diff, Differ},
        ListComparison,
    },
    guild::{AwaitingGuild, ExistingGuild},
};

pub enum ChannelChange {
    Create(AwaitingChannel),
    Update(ExistingChannel, AwaitingChannel, Vec<Diff>),
    Delete(ExistingChannel),
}

pub struct ChannelChangesService {}

impl ChannelChangesService {
    pub fn list_changes(
        &self,
        existing_guild: &ExistingGuild,
        awaiting_guild: &AwaitingGuild,
    ) -> Vec<ChannelChange> {
        let ListComparison {
            extra_self: extra_awaiting,
            extra_other: extra_existing,
            same,
        } = awaiting_guild
            .channels
            .items
            .compare_by_unique_name(&existing_guild.channels);

        let to_create = extra_awaiting
            .into_iter()
            .map(|awaiting| ChannelChange::Create(awaiting.clone()));
        let to_update = same.into_iter().filter_map(|(awaiting, existing)| {
            let diffs = existing.diffs_with(awaiting);
            match diffs.is_empty() {
                true => None,
                false => Some(ChannelChange::Update(
                    existing.clone(),
                    awaiting.clone(),
                    diffs,
                )),
            }
        });
        let to_delete = extra_existing
            .into_iter()
            .map(|existing| ChannelChange::Delete(existing.clone()));

        to_create.chain(to_update).chain(to_delete).collect()
    }
}
