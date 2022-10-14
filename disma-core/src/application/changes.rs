use crate::{
    diff::{base::EntityChange, event::DiffEventListenerRef, factory::DiffCommandFactoryRef},
    guild::{AwaitingGuild, GuildCommanderRef, GuildQuerierRef},
};

pub struct ChangesService {
    guild_commander: GuildCommanderRef,
    command_factory: DiffCommandFactoryRef,
    guild_querier: GuildQuerierRef,
    event_listener: DiffEventListenerRef,
}

impl ChangesService {
    pub fn new(
        guild_commander: GuildCommanderRef,
        command_factory: DiffCommandFactoryRef,
        guild_querier: GuildQuerierRef,
        event_listener: DiffEventListenerRef,
    ) -> Self {
        Self {
            guild_commander,
            command_factory,
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

        let role_diffs = self
            .command_factory
            .for_roles(&existing_guild, awaiting_guild);

        let category_diffs = self
            .command_factory
            .for_categories(&existing_guild, awaiting_guild);

        role_diffs
            .into_iter()
            .chain(category_diffs.into_iter())
            .map(|diff| diff.describe())
            .collect()
    }

    pub fn apply_changes(&self, guild_id: &str, awaiting_guild: &AwaitingGuild) {
        let existing_guild = self.guild_querier.get_guild(guild_id);

        let role_diffs = self
            .command_factory
            .for_roles(&existing_guild, awaiting_guild);

        for diff in role_diffs {
            self.event_listener.before_change_executed(diff.describe());
            diff.execute(&self.guild_commander);
            self.event_listener.after_change_executed(diff.describe());
        }

        let existing_guild = self.guild_querier.get_guild(guild_id);

        let category_diffs = self
            .command_factory
            .for_categories(&existing_guild, awaiting_guild);

        for diff in category_diffs {
            self.event_listener.before_change_executed(diff.describe());
            diff.execute(&self.guild_commander);
            self.event_listener.after_change_executed(diff.describe());
        }
    }
}
