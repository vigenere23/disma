use std::sync::Arc;

use crate::domain::{
    guild::AwaitingGuild,
    role::{AwaitingRole, ExistingRole},
};

use super::GuildCommand;

pub struct AddRole {
    guild: Arc<dyn AwaitingGuild>,
    role: AwaitingRole,
}

impl AddRole {
    fn new(guild: Arc<dyn AwaitingGuild>, role: AwaitingRole) -> Self {
        AddRole { guild, role }
    }
}

impl GuildCommand for AddRole {
    fn execute(&self) {
        self.guild.add_role(&self.role);
    }

    fn describe(&self) -> String {
        format!("Adding role {}.", &self.role.name)
    }
}

pub struct DeleteRole {
    guild: Arc<dyn AwaitingGuild>,
    role: ExistingRole,
}

impl DeleteRole {
    fn new(guild: Arc<dyn AwaitingGuild>, role: ExistingRole) -> Self {
        DeleteRole { guild, role }
    }
}

impl GuildCommand for DeleteRole {
    fn execute(&self) {
        self.guild.delete_role(&self.role.id);
    }

    fn describe(&self) -> String {
        format!("Deleting role {}.", &self.role.name)
    }
}
