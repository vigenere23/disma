use std::collections::HashSet;

pub trait Role: Clone {
    fn name(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq)]
pub struct RolesList<R>
where
    R: Role,
{
    items: Vec<R>,
}

pub struct ListComparison<R1, R2> {
    pub extra_self: Vec<R1>,
    pub extra_other: Vec<R2>,
    pub same: Vec<(R1, R2)>,
}

impl<R: Role> RolesList<R> {
    pub fn find_by_name(&self, name: &str) -> Option<&R> {
        self.items.iter().find(|role| role.name() == name)
    }

    pub fn to_list(&self) -> &Vec<R> {
        &self.items
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

impl<R: Role> From<Vec<R>> for RolesList<R> {
    fn from(roles: Vec<R>) -> Self {
        let mut role_names: HashSet<String> = HashSet::new();

        for role in roles.iter() {
            if role_names.contains(role.name()) {
                panic!("All roles must have unique names.");
            }

            role_names.insert(role.name().to_string());
        }

        Self { items: roles }
    }
}
