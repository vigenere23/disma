use crate::{
    domain::entities::role::{AwaitingRole, ExistingRole},
    guild::GuildCommanderRef,
};

use super::base::{Diff, DiffCommand, Entity, EntityChange};

pub struct AddRole {
    role: AwaitingRole,
}

impl AddRole {
    pub fn new(role: AwaitingRole) -> Self {
        Self { role }
    }
}

impl DiffCommand for AddRole {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.add_role(&self.role);
    }

    fn describe(&self) -> EntityChange {
        EntityChange::Create(Entity::Role, self.role.name.clone())
    }
}

pub struct UpdateRole {
    existing_role: ExistingRole,
    awaiting_role: AwaitingRole,
    diffs: Vec<Diff>,
}

impl UpdateRole {
    pub fn new(existing_role: ExistingRole, awaiting_role: AwaitingRole, diffs: Vec<Diff>) -> Self {
        Self {
            existing_role,
            awaiting_role,
            diffs,
        }
    }
}

impl DiffCommand for UpdateRole {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.update_role(&self.existing_role.id, &self.awaiting_role);
    }

    fn describe(&self) -> EntityChange {
        EntityChange::Update(
            Entity::Role,
            self.existing_role.name.clone(),
            self.diffs.clone(),
        )
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

impl DiffCommand for DeleteRole {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.delete_role(&self.role.id);
    }

    fn describe(&self) -> EntityChange {
        EntityChange::Delete(Entity::Role, self.role.name.clone())
    }
}
