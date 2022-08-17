use std::sync::Arc;

use crate::domain::{
    guild::GuildCommander,
    role::{AwaitingRole, ExistingRole},
};

use super::GuildCommand;

pub struct AddRole {
    guild_commander: Arc<dyn GuildCommander>,
    role: AwaitingRole,
}

impl AddRole {
    pub fn new(guild: Arc<dyn GuildCommander>, role: AwaitingRole) -> Self {
        Self {
            guild_commander: guild,
            role,
        }
    }
}

impl GuildCommand for AddRole {
    fn execute(&self) {
        self.guild_commander.add_role(&self.role);
    }

    fn describe(&self) -> String {
        format!("Adding role {}", &self.role.name)
    }
}

pub struct DeleteRole {
    guild_commander: Arc<dyn GuildCommander>,
    role: ExistingRole,
}

impl DeleteRole {
    pub fn new(guild: Arc<dyn GuildCommander>, role: ExistingRole) -> Self {
        Self {
            guild_commander: guild,
            role,
        }
    }
}

impl GuildCommand for DeleteRole {
    fn execute(&self) {
        self.guild_commander.delete_role(&self.role.id);
    }

    fn describe(&self) -> String {
        format!("Deleting role {}", &self.role.name)
    }
}

pub struct UpdateRole {
    guild_commander: Arc<dyn GuildCommander>,
    existing_role: ExistingRole,
    awaiting_role: AwaitingRole,
}

impl UpdateRole {
    pub fn new(
        guild: Arc<dyn GuildCommander>,
        existing_role: ExistingRole,
        awaiting_role: AwaitingRole,
    ) -> Self {
        Self {
            guild_commander: guild,
            existing_role,
            awaiting_role,
        }
    }
}

impl GuildCommand for UpdateRole {
    fn execute(&self) {
        self.guild_commander
            .update_role(&self.existing_role.id, &self.awaiting_role);
    }

    fn describe(&self) -> String {
        format!(
            "Updating role {}\nfrom :{:#?}\nto :{:#?}",
            &self.existing_role.name, &self.existing_role, &self.awaiting_role
        )
    }
}
