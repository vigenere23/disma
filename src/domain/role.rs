use super::permission::PermissionsList;

#[derive(Debug, Clone)]
pub struct ExistingRole {
    pub id: String,
    pub name: String,
    pub permissions: PermissionsList,
    pub is_mentionalbe: bool,
    pub show_in_sidebar: bool,
}

impl PartialEq<AwaitingRole> for ExistingRole {
    fn eq(&self, other: &AwaitingRole) -> bool {
        self.name == other.name
            && self.is_mentionalbe == other.is_mentionalbe
            && self.show_in_sidebar == other.show_in_sidebar
            && self.permissions == other.permissions
    }
}

#[derive(Debug, Clone)]
pub struct AwaitingRole {
    pub name: String,
    pub permissions: PermissionsList,
    pub is_mentionalbe: bool,
    pub show_in_sidebar: bool,
}

impl PartialEq<ExistingRole> for AwaitingRole {
    fn eq(&self, other: &ExistingRole) -> bool {
        self.name == other.name
            && self.is_mentionalbe == other.is_mentionalbe
            && self.show_in_sidebar == other.show_in_sidebar
            && self.permissions == other.permissions
    }
}

#[derive(Debug)]
pub struct ExistingRolesList {
    roles: Vec<ExistingRole>,
}

impl ExistingRolesList {
    pub fn new(roles: Vec<ExistingRole>) -> Self {
        Self { roles }
    }

    pub fn find_by_name(&self, name: &str) -> Option<&ExistingRole> {
        (&self.roles).into_iter().find(|role| role.name == name)
    }

    pub fn items(&self) -> &Vec<ExistingRole> {
        &self.roles
    }
}

// TODO merge with existing roles list
pub struct AwaitingRolesList {
    roles: Vec<AwaitingRole>,
}

impl From<&Vec<AwaitingRole>> for AwaitingRolesList {
    fn from(roles: &Vec<AwaitingRole>) -> Self {
        Self {
            roles: roles.clone(),
        }
    }
}

impl AwaitingRolesList {
    pub fn find_by_name(&self, name: &str) -> Option<&AwaitingRole> {
        (&self.roles).into_iter().find(|role| role.name == name)
    }

    pub fn items(&self) -> &Vec<AwaitingRole> {
        &self.roles
    }
}
