use core::fmt::Debug;
use std::sync::Arc;

use crate::{
    commands::{Command, CommandDescription, CommandEntity, CommandFactory, CommandRef},
    diff::{Diff, Differ},
    guild::{ExistingGuild, GuildCommanderRef},
};

use super::{AwaitingRole, AwaitingRolesList, ExistingRole};

pub trait ExtraRolesStrategyTrait {}

impl CommandFactory for AwaitingRolesList {
    fn commands_for(&self, existing_guild: &ExistingGuild) -> Vec<CommandRef> {
        let mut commands: Vec<CommandRef> = Vec::new();

        for awaiting_role in self.items.to_list() {
            match existing_guild.roles.find_by_name(&awaiting_role.name) {
                Some(existing_role) => {
                    if existing_role != awaiting_role {
                        let command = UpdateRole::new(
                            existing_role.clone(),
                            awaiting_role.clone(),
                            existing_role.diffs_with(awaiting_role),
                        );
                        commands.push(Arc::from(command));
                    }
                }
                None => {
                    let command = AddRole::new(awaiting_role.clone());
                    commands.push(Arc::from(command));
                }
            }
        }

        for existing_role in existing_guild.roles.to_list() {
            if self.items.find_by_name(&existing_role.name).is_none() {
                self.extra_items_strategy
                    .handle_extra_role(existing_role, &mut commands);
            }
        }

        commands
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

pub trait ExtraRolesStrategy {
    fn _type(&self) -> ExtraRolesStrategyType;
    fn handle_extra_role(&self, extra_role: &ExistingRole, commands: &mut Vec<CommandRef>);
}

#[derive(Debug, PartialEq)]
pub enum ExtraRolesStrategyType {
    Keep,
    Remove,
}

impl Debug for dyn ExtraRolesStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self._type())
    }
}

pub struct RemoveExtraRoles {}

impl ExtraRolesStrategy for RemoveExtraRoles {
    fn _type(&self) -> ExtraRolesStrategyType {
        ExtraRolesStrategyType::Remove
    }

    fn handle_extra_role(&self, extra_role: &ExistingRole, commands: &mut Vec<CommandRef>) {
        let command = DeleteRole::new(extra_role.clone());
        commands.push(Arc::from(command));
    }
}

pub struct KeepExtraRoles {}

impl ExtraRolesStrategy for KeepExtraRoles {
    fn _type(&self) -> ExtraRolesStrategyType {
        ExtraRolesStrategyType::Keep
    }

    fn handle_extra_role(&self, _extra_role: &ExistingRole, _commands: &mut Vec<CommandRef>) {}
}
