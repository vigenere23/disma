use crate::{
    diff::{base::EntityChange, differ::GuildDifferRef, event::DiffEventListenerRef},
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

    pub fn list_diffs(&self, guild_id: &str, awaiting_guild: &AwaitingGuild) -> Vec<EntityChange> {
        let existing_guild = self.guild_querier.get_guild(guild_id);

        let role_diffs = self
            .guild_differ
            .calculate_role_diffs(&existing_guild, awaiting_guild);

        let category_diffs = self
            .guild_differ
            .calculate_category_diffs(&existing_guild, awaiting_guild);

        role_diffs
            .into_iter()
            .chain(category_diffs.into_iter())
            .map(|diff| diff.describe())
            .collect()
    }

    pub fn apply_diffs(&self, guild_id: &str, awaiting_guild: &AwaitingGuild) {
        let existing_guild = self.guild_querier.get_guild(guild_id);

        let role_diffs = self
            .guild_differ
            .calculate_role_diffs(&existing_guild, awaiting_guild);

        for diff in role_diffs {
            diff.execute(&self.guild_commander);
            self.diff_event_listener
                .after_change_executed(diff.describe());
        }

        let existing_guild = self.guild_querier.get_guild(guild_id);

        let category_diffs = self
            .guild_differ
            .calculate_category_diffs(&existing_guild, awaiting_guild);

        for diff in category_diffs {
            diff.execute(&self.guild_commander);
            self.diff_event_listener
                .after_change_executed(diff.describe());
        }
    }
}
