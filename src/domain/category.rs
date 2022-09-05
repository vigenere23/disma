// pub struct AwaitingCategory {
//     name: String,
//     channels: Vec<AwaitingChannel>,
//     //permission_overwrites: Option<Vec<PermissionOverwritesDto>>,
// }

use super::{
    permission::PermissionsList,
    role::{ExistingRole, Role},
};

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
