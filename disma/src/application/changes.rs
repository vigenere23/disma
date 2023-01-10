use crate::{
    commands::{
        CommandDescription, CommandEventListenerRef, CommandEventType, CommandFactory, CommandRef,
    },
    guild::{AwaitingGuild, GuildCommanderRef, GuildQuerierRef},
};

pub struct ChangesService {
    guild_commander: GuildCommanderRef,
    guild_querier: GuildQuerierRef,
    event_listener: CommandEventListenerRef,
}

impl ChangesService {
    pub fn new(
        guild_commander: GuildCommanderRef,
        guild_querier: GuildQuerierRef,
        event_listener: CommandEventListenerRef,
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
    ) -> Vec<CommandDescription> {
        let existing_guild = self.guild_querier.get_guild(guild_id);

        let role_commands = awaiting_guild.roles.commands_for(&existing_guild);
        let category_commands = awaiting_guild.categories.commands_for(&existing_guild);
        let channel_commands = awaiting_guild.channels.commands_for(&existing_guild);

        role_commands
            .into_iter()
            .chain(category_commands.into_iter())
            .chain(channel_commands.into_iter())
            .map(|command| command.describe())
            .collect()
    }

    pub fn apply_changes(&self, guild_id: &str, awaiting_guild: &AwaitingGuild) {
        self.apply_changes_for(guild_id, &awaiting_guild.roles);
        self.apply_changes_for(guild_id, &awaiting_guild.categories);
        self.apply_changes_for(guild_id, &awaiting_guild.channels);
    }

    fn apply_changes_for(&self, guild_id: &str, command_factory: &dyn CommandFactory) {
        let existing_guild = self.guild_querier.get_guild(guild_id);
        command_factory
            .commands_for(&existing_guild)
            .into_iter()
            .for_each(|command| self.apply_command(command));
    }

    fn apply_command(&self, command: CommandRef) {
        let description = command.describe();
        self.event_listener
            .handle(CommandEventType::BeforeExecution, description.clone());

        command.execute(&self.guild_commander);

        self.event_listener
            .handle(CommandEventType::AfterExecution, description);
    }
}
