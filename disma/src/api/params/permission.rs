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
    pub fn into<R>(self, roles: &RolesList<R>) -> PermissionsOverwrite
    where
        R: Role,
    {
        PermissionsOverwrite {
            name: roles
                .find_by_name(&self.role)
                .unwrap_or_else(|| {
                    panic!(
                        "Cannot build permissions overwrite from non-existant role '{}'",
                        &self.role
                    )
                })
                .name()
                .to_string(),
            allow: PermissionsList::from(self.allow),
            deny: PermissionsList::from(self.deny),
        }
    }
}

impl From<&PermissionsOverwrite> for PermissionsOverwriteParams {
    fn from(permissions: &PermissionsOverwrite) -> Self {
        Self {
            role: permissions.name.clone(),
            allow: permissions.allow.to_list(),
            deny: permissions.deny.to_list(),
        }
    }
}
