use core::fmt::Debug;

use crate::core::changes::role::RoleChange;

use super::ExistingRole;

pub trait ExtraRolesStrategyTrait {}

pub trait ExtraRolesStrategy {
    fn _type(&self) -> ExtraRolesStrategyType;
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

    fn handle_extra_role(&self, extra_existing: &ExistingRole, changes: &mut Vec<RoleChange>) {
        changes.push(RoleChange::Delete(extra_existing.clone()));
    }
}

pub struct KeepExtraRoles {}

impl ExtraRolesStrategy for KeepExtraRoles {
    fn _type(&self) -> ExtraRolesStrategyType {
        ExtraRolesStrategyType::Keep
    }

    fn handle_extra_role(&self, _extra_existing: &ExistingRole, _changes: &mut Vec<RoleChange>) {}
}

#[cfg(test)]
mod tests {
    // TODO
}
