use std::sync::Arc;

use crate::domain::entities::{
    guild::GuildCommander,
    role::{AwaitingRole, ExistingRole},
};

use super::base::{Diff, DiffDescription};

pub struct AddRole {
    role: AwaitingRole,
}

impl AddRole {
    pub fn new(role: AwaitingRole) -> Self {
        Self { role }
    }
}

impl Diff for AddRole {
    fn execute(&self, guild: Arc<dyn GuildCommander>) {
        guild.add_role(&self.role);
    }

    fn describe(&self) -> DiffDescription {
        DiffDescription {
            summary: format!("🆕 Adding role {}", &self.role.name),
            details: vec![],
        }
    }
}

pub struct UpdateRole {
    existing_role: ExistingRole,
    awaiting_role: AwaitingRole,
}

impl UpdateRole {
    pub fn new(existing_role: ExistingRole, awaiting_role: AwaitingRole) -> Self {
        Self {
            existing_role,
            awaiting_role,
        }
    }
}

impl Diff for UpdateRole {
    fn execute(&self, guild: Arc<dyn GuildCommander>) {
        guild.update_role(&self.existing_role.id, &self.awaiting_role);
    }

    fn describe(&self) -> DiffDescription {
        DiffDescription {
            summary: format!(
                "🔄 Updating role {}\nfrom :{:#?}\nto :{:#?}",
                &self.existing_role.name, &self.existing_role, &self.awaiting_role
            ),
            details: vec![],
        }
    }
}

pub struct DeleteRole {
    role: ExistingRole,
}

impl DeleteRole {
    pub fn new(role: ExistingRole) -> Self {
        Self { role }
    }
}

impl Diff for DeleteRole {
    fn execute(&self, guild: Arc<dyn GuildCommander>) {
        guild.delete_role(&self.role.id);
    }

    fn describe(&self) -> DiffDescription {
        DiffDescription {
            summary: format!("🗑️  Deleting role {}", &self.role.name),
            details: vec![],
        }
    }
}
