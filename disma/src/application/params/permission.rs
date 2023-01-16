use crate::{
    permission::{Permission, PermissionsList, PermissionsOverwrite},
    role::{Role, RolesList},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PermissionsOverwriteParams {
    pub role: String,
    #[serde(default = "Vec::default")]
    pub allow: Vec<Permission>,
    #[serde(default = "Vec::default")]
    pub deny: Vec<Permission>,
}

impl PermissionsOverwriteParams {
    pub fn into<R>(self, roles: &RolesList<R>) -> PermissionsOverwrite<R>
    where
        R: Role,
    {
        PermissionsOverwrite {
            role: roles
                .find_by_name(&self.role)
                .unwrap_or_else(|| panic!("No role found with name {}", &self.role))
                .clone(),
            allow: PermissionsList::from(self.allow),
            deny: PermissionsList::from(self.deny),
        }
    }
}

impl<R> From<&PermissionsOverwrite<R>> for PermissionsOverwriteParams
where
    R: Role,
{
    fn from(permissions: &PermissionsOverwrite<R>) -> Self {
        Self {
            role: permissions.role.name(),
            allow: permissions.allow.to_list(),
            deny: permissions.deny.to_list(),
        }
    }
}
