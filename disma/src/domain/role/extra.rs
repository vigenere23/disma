use core::fmt::Debug;
use std::sync::Arc;

use crate::core::{
    changes::role::RoleChange,
    commands::{role::DeleteRole, CommandRef},
};

use super::ExistingRole;

pub trait ExtraRolesStrategyTrait {}

pub trait ExtraRolesStrategy {
    fn _type(&self) -> ExtraRolesStrategyType;
    fn handle_extra_role_commands(
        &self,
        extra_existing: &ExistingRole,
        commands: &mut Vec<CommandRef>,
    );
    fn handle_extra_role(&self, extra_existing: &ExistingRole, changes: &mut Vec<RoleChange>);
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

    fn handle_extra_role_commands(
        &self,
        extra_existing: &ExistingRole,
        commands: &mut Vec<CommandRef>,
    ) {
        let command = DeleteRole::new(extra_existing.clone());
        commands.push(Arc::from(command));
    }

    fn handle_extra_role(&self, extra_existing: &ExistingRole, changes: &mut Vec<RoleChange>) {
        changes.push(RoleChange::Delete(extra_existing.clone()));
    }
}

pub struct KeepExtraRoles {}

impl ExtraRolesStrategy for KeepExtraRoles {
    fn _type(&self) -> ExtraRolesStrategyType {
        ExtraRolesStrategyType::Keep
    }

    fn handle_extra_role_commands(
        &self,
        _extra_existing: &ExistingRole,
        _commands: &mut Vec<CommandRef>,
    ) {
    }

    fn handle_extra_role(&self, _extra_existing: &ExistingRole, _changes: &mut Vec<RoleChange>) {}
}
