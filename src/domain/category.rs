use super::{
    permission::PermissionsList,
    role::{AwaitingRole, ExistingRole, Role},
};

pub struct AwaitingCategory {
    pub name: String,
    pub permission_overwrites: Option<Vec<CategoryRolePermissions<AwaitingRole>>>,
    // pub channels: Vec<AwaitingChannel>,
}

#[derive(Debug)]
pub struct ExistingCategory {
    pub id: String,
    pub name: String,
    pub permissions: Option<Vec<CategoryRolePermissions<ExistingRole>>>,
}

#[derive(Debug)]
pub struct CategoryRolePermissions<T>
where
    T: Role,
{
    pub role: T,
    pub allow: PermissionsList,
    pub deny: PermissionsList,
}
