#[derive(Debug)]
pub struct ExistingRole {
    pub id: String,
    pub name: String,
}

pub struct AwaitingRole {
    pub name: String,
}

#[derive(Debug)]
pub struct RolesList {
    roles: Vec<ExistingRole>,
}

impl RolesList {
    pub fn new(roles: Vec<ExistingRole>) -> Self {
        RolesList { roles }
    }

    pub fn find_by_name(&self, name: String) -> ExistingRole {
        todo!()
    }
}
