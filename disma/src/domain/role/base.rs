use std::collections::HashMap;

use crate::core::ListComparison;

use super::ExistingRole;

pub trait Role: Clone {
    fn name(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq)]
pub struct RolesList<R>
where
    R: Role,
{
    roles_by_name: HashMap<String, R>,
}

impl<R: Role> RolesList<R> {
    pub fn new() -> Self {
        Self {
            roles_by_name: HashMap::new(),
        }
    }

    pub fn find_by_name(&self, name: &str) -> Option<&R> {
        self.roles_by_name.get(name)
    }

    pub fn add(&mut self, role: R) {
        if self.roles_by_name.contains_key(role.name()) {
            // TODO replace with Result
            panic!("All roles must have unique names.");
        }

        self.roles_by_name.insert(role.name().to_string(), role);
    }

    pub fn to_list(&self) -> Vec<&R> {
        self.roles_by_name.values().collect()
    }

    pub fn compare_by_name<'a, R2: Role>(
        &'a self,
        other: &'a RolesList<R2>,
    ) -> ListComparison<&R, &R2> {
        let mut extra_self: Vec<&R> = Vec::new();
        let mut extra_other: Vec<&R2> = Vec::new();
        let mut same: Vec<(&R, &R2)> = Vec::new();

        for self_item in self.to_list() {
            match other.find_by_name(self_item.name()) {
                Some(other_item) => same.push((self_item, other_item)),
                None => extra_self.push(self_item),
            }
        }

        for other_item in other.to_list() {
            if self.find_by_name(other_item.name()).is_none() {
                extra_other.push(other_item)
            }
        }

        ListComparison {
            extra_self,
            extra_other,
            same,
        }
    }
}

impl<R: Role> Default for RolesList<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: Role> From<Vec<R>> for RolesList<R> {
    fn from(roles: Vec<R>) -> Self {
        let mut roles_list = RolesList::new();

        for role in roles.into_iter() {
            roles_list.add(role);
        }

        roles_list
    }
}

impl RolesList<ExistingRole> {
    pub fn find_by_id(&self, id: &str) -> Option<&ExistingRole> {
        self.to_list().into_iter().find(|role| role.id == id)
    }

    pub fn add_or_replace(&mut self, role: ExistingRole) {
        self.roles_by_name.insert(role.name().to_string(), role);
    }

    pub fn remove(&mut self, role: ExistingRole) {
        self.roles_by_name.remove(role.name());
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::ListComparison, role::ExistingRole, tests::fixtures::existing::ExistingRoleFixture,
    };

    use super::RolesList;

    const SOME_NAME: &str = "non-existant";
    const SOME_ID: &str = "non-existant";

    #[test]
    fn can_find_by_name() {
        let role = ExistingRoleFixture::new().build();
        let list = RolesList::from(vec![role.clone()]);

        let found = list.find_by_name(&role.name);

        assert!(found.is_some());
        assert_eq!(found.unwrap().to_owned(), role);
    }

    #[test]
    fn given_role_not_in_list_when_finding_by_name_should_return_none() {
        let list = RolesList::<ExistingRole>::new();

        let found = list.find_by_name(SOME_NAME);

        assert!(found.is_none());
    }

    #[test]
    fn can_find_by_id() {
        let role = ExistingRoleFixture::new().build();
        let list = RolesList::from(vec![role.clone()]);

        let found = list.find_by_id(&role.id);

        assert!(found.is_some());
        assert_eq!(found.unwrap().to_owned(), role);
    }

    #[test]
    fn given_role_not_in_list_when_finding_by_id_should_return_none() {
        let list = RolesList::<ExistingRole>::new();

        let found = list.find_by_id(SOME_ID);

        assert!(found.is_none());
    }

    #[test]
    fn can_add_role() {
        let role = ExistingRoleFixture::new().build();
        let mut list = RolesList::<ExistingRole>::new();

        list.add(role.clone());

        assert_eq!(list.to_list(), vec![&role]);
    }

    #[test]
    #[should_panic]
    fn given_role_with_same_name_already_in_list_when_adding_role_should_panics() {
        let role = ExistingRoleFixture::new().with_name(SOME_NAME).build();
        let role_copy = ExistingRoleFixture::new().with_name(SOME_NAME).build();
        let mut list = RolesList::from(vec![role]);

        list.add(role_copy);
    }

    #[test]
    fn given_role_not_in_list_when_adding_or_replacing_should_add() {
        let role = ExistingRoleFixture::new().build();
        let mut list = RolesList::<ExistingRole>::new();

        list.add_or_replace(role.clone());

        assert_eq!(list.to_list(), vec![&role]);
    }

    #[test]
    fn given_role_already_in_list_when_adding_or_replacing_should_replace_according_to_name() {
        let role = ExistingRoleFixture::new().with_name(SOME_NAME).build();
        let role_clone = ExistingRoleFixture::new().with_name(SOME_NAME).build();
        let mut list = RolesList::from(vec![role]);

        list.add_or_replace(role_clone.clone());

        assert_eq!(list.to_list(), vec![&role_clone]);
    }

    #[test]
    fn can_remove_role() {
        let role = ExistingRoleFixture::new().build();
        let mut list = RolesList::from(vec![role.clone()]);

        list.remove(role);

        assert_eq!(list.to_list(), Vec::<&ExistingRole>::new());
    }

    #[test]
    fn given_role_not_in_list_when_removing_should_do_nothing() {
        let non_existant_role = ExistingRoleFixture::new().build();
        let mut list = RolesList::<ExistingRole>::new();

        list.remove(non_existant_role);
    }

    #[test]
    fn can_compare_lists_by_role_names() {
        let extra_self_role = ExistingRoleFixture::new().build();
        let extra_other_role = ExistingRoleFixture::new().build();
        let same_self_role = ExistingRoleFixture::new().with_name(SOME_NAME).build();
        let same_other_role = ExistingRoleFixture::new().with_name(SOME_NAME).build();

        let self_list = RolesList::from(vec![same_self_role.clone(), extra_self_role.clone()]);
        let other_list = RolesList::from(vec![same_other_role.clone(), extra_other_role.clone()]);

        let ListComparison {
            extra_self,
            extra_other,
            same,
        } = self_list.compare_by_name(&other_list);

        assert_eq!(extra_self, vec![&extra_self_role]);
        assert_eq!(extra_other, vec![&extra_other_role]);
        assert_eq!(same, vec![(&same_self_role, &same_other_role)]);
    }
}
