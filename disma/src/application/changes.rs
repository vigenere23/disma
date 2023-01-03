use crate::{
    diff::{
        base::{DiffCommandFactory, EntityChange},
        event::DiffEventListenerRef,
    },
    guild::{AwaitingGuild, GuildCommanderRef, GuildQuerierRef},
};

pub struct ChangesService {
    guild_commander: GuildCommanderRef,
    guild_querier: GuildQuerierRef,
    event_listener: DiffEventListenerRef,
}

impl ChangesService {
    pub fn new(
        guild_commander: GuildCommanderRef,
        guild_querier: GuildQuerierRef,
        event_listener: DiffEventListenerRef,
    ) -> Self {
        Self {
            guild_commander,
            guild_querier,
            event_listener,
        }
    }

    pub fn list_changes(
        &self,
        guild_id: &str,
        awaiting_guild: &AwaitingGuild,
    ) -> Vec<EntityChange> {
        let existing_guild = self.guild_querier.get_guild(guild_id);

        let role_diffs = awaiting_guild.roles.diff_commands_for(&existing_guild);

        let category_diffs = awaiting_guild.categories.diff_commands_for(&existing_guild);

        let channel_diffs = awaiting_guild.channels.diff_commands_for(&existing_guild);

        role_diffs
            .into_iter()
            .chain(category_diffs.into_iter())
            .chain(channel_diffs.into_iter())
            .map(|diff| diff.describe())
            .collect()
    }

    pub fn apply_changes(&self, guild_id: &str, awaiting_guild: &AwaitingGuild) {
        let existing_guild = self.guild_querier.get_guild(guild_id);

        let role_diffs = awaiting_guild.roles.diff_commands_for(&existing_guild);

        for diff in role_diffs {
            self.event_listener.before_change_executed(diff.describe());
            diff.execute(&self.guild_commander);
            self.event_listener.after_change_executed(diff.describe());
        }

        let existing_guild = self.guild_querier.get_guild(guild_id);

        let category_diffs = awaiting_guild.categories.diff_commands_for(&existing_guild);

        for diff in category_diffs {
            self.event_listener.before_change_executed(diff.describe());
            diff.execute(&self.guild_commander);
            self.event_listener.after_change_executed(diff.describe());
        }

        let existing_guild = self.guild_querier.get_guild(guild_id);

        let channel_diffs = awaiting_guild.channels.diff_commands_for(&existing_guild);

        for diff in channel_diffs {
            self.event_listener.before_change_executed(diff.describe());
            diff.execute(&self.guild_commander);
            self.event_listener.after_change_executed(diff.describe());
        }
    }
}
