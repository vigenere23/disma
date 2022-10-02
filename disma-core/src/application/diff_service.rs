use crate::{
    diff::{base::Diff, differ::GuildDifferRef, event::DiffEventListenerRef},
    guild::{AwaitingGuild, GuildCommanderRef, GuildQuerierRef},
};

pub struct GuildDiffService {
    guild_commander: GuildCommanderRef,
    guild_differ: GuildDifferRef,
    guild_querier: GuildQuerierRef,
    diff_event_listener: DiffEventListenerRef,
}

impl GuildDiffService {
    pub fn new(
        guild_commander: GuildCommanderRef,
        guild_differ: GuildDifferRef,
        guild_querier: GuildQuerierRef,
        diff_event_listener: DiffEventListenerRef,
    ) -> Self {
        Self {
            guild_commander,
            guild_differ,
            guild_querier,
            diff_event_listener,
        }
    }

    pub fn list_diffs(&self, guild_id: &str, awaiting_guild: &AwaitingGuild) -> Vec<Diff> {
        let existing_guild = self.guild_querier.get_guild(guild_id);

        let diffs = self
            .guild_differ
            .calculate_diffs(&existing_guild, awaiting_guild);

        diffs.into_iter().map(|diff| diff.describe()).collect()
    }

    pub fn apply_diffs(&self, guild_id: &str, awaiting_guild: &AwaitingGuild) {
        let existing_guild = self.guild_querier.get_guild(guild_id);

        let diffs = self
            .guild_differ
            .calculate_diffs(&existing_guild, awaiting_guild);

        for diff in diffs {
            diff.execute(&self.guild_commander);
            self.diff_event_listener
                .after_diff_executed(diff.describe());
        }
    }
}
