use std::sync::Arc;

use crate::domain::{
    commands::{
        roles::{AddRole, DeleteRole, UpdateRole},
        GuildCommand,
    },
    guild::{AwaitingGuild, ExistingGuild, GuildCommander},
};

pub struct DiffCalculator {
    guild_commander: Arc<dyn GuildCommander>,
}

impl DiffCalculator {
    pub fn new(guild_commander: Arc<dyn GuildCommander>) -> Self {
        Self { guild_commander }
    }

    pub fn create_commands(
        &self,
        existing_guild: ExistingGuild,
        awaiting_guild: AwaitingGuild,
    ) -> Vec<Arc<dyn GuildCommand>> {
        let mut commands: Vec<Arc<dyn GuildCommand>> = Vec::new();

        for awaiting_role in awaiting_guild.roles.items() {
            match existing_guild.roles.find_by_name(&awaiting_role.name) {
                Some(role) => {
                    if awaiting_role != role {
                        let command = UpdateRole::new(
                            self.guild_commander.clone(),
                            role.clone(),
                            awaiting_role.clone(),
                        );
                        commands.push(Arc::from(command));
                    }
                }
                None => {
                    let command = AddRole::new(self.guild_commander.clone(), awaiting_role.clone());
                    commands.push(Arc::from(command));
                }
            }
        }

        for existing_role in existing_guild.roles.items() {
            if awaiting_guild
                .roles
                .find_by_name(&existing_role.name)
                .is_none()
            {
                let command = DeleteRole::new(self.guild_commander.clone(), existing_role.clone());
                commands.push(Arc::from(command));
            }
        }

        commands
    }
}
