use std::collections::HashMap;

use crate::base::ListComparison;

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

    pub fn push(&mut self, role: R) {
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
            roles_list.push(role);
        }

        roles_list
    }
}
