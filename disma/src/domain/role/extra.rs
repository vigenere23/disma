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
    use crate::tests::fixtures::existing::ExistingRoleFixture;

    use super::*;

    #[test]
    fn when_keeping_extra_roles_should_not_add_changes() {
        let mut changes: Vec<RoleChange> = Vec::new();
        let extra_role = ExistingRoleFixture::new().build();

        let strategy = KeepExtraRoles {};
        strategy.handle_extra_role(&extra_role, &mut changes);

        assert!(changes.is_empty());
    }

    #[test]
    fn when_removing_extra_roles_should_add_delete_change() {
        let mut changes: Vec<RoleChange> = Vec::new();
        let extra_role = ExistingRoleFixture::new().build();

        let strategy = RemoveExtraRoles {};
        strategy.handle_extra_role(&extra_role, &mut changes);

        assert!(!changes.is_empty());
        assert_eq!(changes, vec![RoleChange::Delete(extra_role)]);
    }
}
