use core::fmt::Debug;
use std::sync::Arc;

use crate::core::commands::{role::DeleteRole, CommandRef};

use super::ExistingRole;

pub trait ExtraRolesStrategyTrait {}

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
