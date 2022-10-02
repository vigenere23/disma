use crate::{
    domain::entities::role::{AwaitingRole, ExistingRole},
    guild::GuildCommanderRef,
};

use super::base::{Diff, DiffCommand};

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

    fn describe(&self) -> Diff {
        Diff::Add(format!("role \"{}\"", &self.role.name))
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

impl DiffCommand for UpdateRole {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.update_role(&self.existing_role.id, &self.awaiting_role);
    }

    fn describe(&self) -> Diff {
        Diff::Update(
            format!("role \"{}\"", &self.existing_role.name),
            vec![
                Diff::Remove(format!("{:#?}", &self.existing_role)), // TODO more granular diffs
                Diff::Add(format!("{:#?}", &self.awaiting_role)),
            ],
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

    fn describe(&self) -> Diff {
        Diff::Remove(format!("role \"{}\"", &self.role.name))
    }
}
