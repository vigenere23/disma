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

impl<R: Role> RolesList<R> {
    pub fn find_by_name(&self, name: &str) -> Option<&R> {
        self.items.iter().find(|role| role.name() == name)
    }

    pub fn to_list(&self) -> &Vec<R> {
        &self.items
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
