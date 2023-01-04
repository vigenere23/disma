use std::sync::Arc;

use crate::{
    commands::{Command, CommandDescription, CommandEntity, CommandFactory, CommandRef},
    diff::{Diff, Differ},
    guild::{ExistingGuild, GuildCommanderRef},
};

use super::{AwaitingRole, AwaitingRolesList, ExistingRole, ExtraRolesStrategy};

pub trait ExtraRolesStrategyTrait {}

impl CommandFactory for AwaitingRolesList {
    fn commands_for(&self, existing_guild: &ExistingGuild) -> Vec<CommandRef> {
        let mut diffs: Vec<CommandRef> = Vec::new();

        for awaiting_role in self.items.to_list() {
            match existing_guild.roles.find_by_name(&awaiting_role.name) {
                Some(existing_role) => {
                    if existing_role != awaiting_role {
                        let command = UpdateRole::new(
                            existing_role.clone(),
                            awaiting_role.clone(),
                            existing_role.diffs_with(awaiting_role),
                        );
                        diffs.push(Arc::from(command));
                    }
                }
                None => {
                    let command = AddRole::new(awaiting_role.clone());
                    diffs.push(Arc::from(command));
                }
            }
        }

        if self.extra_items.strategy == ExtraRolesStrategy::Remove {
            for existing_role in existing_guild.roles.to_list() {
                if self.items.find_by_name(&existing_role.name).is_none() {
                    let command = DeleteRole::new(existing_role.clone());
                    diffs.push(Arc::from(command));
                }
            }
        }

        diffs
    }
}

pub struct AddRole {
    role: AwaitingRole,
}

impl AddRole {
    pub fn new(role: AwaitingRole) -> Self {
        Self { role }
    }
}

impl Command for AddRole {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.add_role(&self.role);
    }

    fn describe(&self) -> CommandDescription {
        CommandDescription::Create(CommandEntity::Role, self.role.name.clone())
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

impl Command for UpdateRole {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.update_role(&self.existing_role.id, &self.awaiting_role);
    }

    fn describe(&self) -> CommandDescription {
        CommandDescription::Update(
            CommandEntity::Role,
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

impl Command for DeleteRole {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.delete_role(&self.role.id);
    }

    fn describe(&self) -> CommandDescription {
        CommandDescription::Delete(CommandEntity::Role, self.role.name.clone())
    }
}
