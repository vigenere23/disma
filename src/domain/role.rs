#[derive(Debug, Clone)]
pub struct ExistingRole {
    pub id: String,
    pub name: String,
    pub is_mentionalbe: bool,
    pub show_in_sidebar: bool,
}

#[derive(Clone)]
pub struct AwaitingRole {
    pub name: String,
    pub is_mentionalbe: bool,
    pub show_in_sidebar: bool,
}

#[derive(Debug)]
pub struct ExistingRolesList {
    pub items: Vec<ExistingRole>,
}

impl ExistingRolesList {
    pub fn new(roles: Vec<ExistingRole>) -> Self {
        Self { items: roles }
    }

    pub fn find_by_name(&self, name: &str) -> Option<&ExistingRole> {
        (&self.items).into_iter().find(|role| role.name == name)
    }
}

pub struct AwaitingRolesList {
    pub items: Vec<AwaitingRole>,
}

impl AwaitingRolesList {
    pub fn new(roles: Vec<AwaitingRole>) -> Self {
        Self { items: roles }
    }

    pub fn find_by_name(&self, name: &str) -> Option<&AwaitingRole> {
        (&self.items).into_iter().find(|role| role.name == name)
    }
}
