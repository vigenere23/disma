use crate::{
    commands::{CommandDescription, CommandEventListenerRef, CommandFactory},
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
        let existing_guild = self.guild_querier.get_guild(guild_id);

        let role_commands = awaiting_guild.roles.commands_for(&existing_guild);

        for command in role_commands {
            self.event_listener
                .before_command_execution(command.describe());
            command.execute(&self.guild_commander);
            self.event_listener
                .after_command_execution(command.describe());
        }

        let existing_guild = self.guild_querier.get_guild(guild_id);

        let category_commands = awaiting_guild.categories.commands_for(&existing_guild);

        for command in category_commands {
            self.event_listener
                .before_command_execution(command.describe());
            command.execute(&self.guild_commander);
            self.event_listener
                .after_command_execution(command.describe());
        }

        let existing_guild = self.guild_querier.get_guild(guild_id);

        let channel_commands = awaiting_guild.channels.commands_for(&existing_guild);

        for command in channel_commands {
            self.event_listener
                .before_command_execution(command.describe());
            command.execute(&self.guild_commander);
            self.event_listener
                .after_command_execution(command.describe());
        }
    }
}
