use disma::{
    permission::{PermissionsList, PermissionsOverwrites},
    role::{Role, RolesList},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PermissionsOverwritesConfig {
    pub role: String,
    #[serde(default = "Vec::default")]
    pub allow: Vec<String>,
    #[serde(default = "Vec::default")]
    pub deny: Vec<String>,
}

impl PermissionsOverwritesConfig {
    pub fn into<R>(self, roles: &RolesList<R>) -> PermissionsOverwrites<R>
    where
        R: Role,
    {
        PermissionsOverwrites {
            role: roles
                .find_by_name(&self.role)
                .unwrap_or_else(|| panic!("No role found with name {}", &self.role))
                .clone(),
            allow: PermissionsList::from(self.allow),
            deny: PermissionsList::from(self.deny),
        }
    }
}

impl<R> From<&PermissionsOverwrites<R>> for PermissionsOverwritesConfig
where
    R: Role,
{
    fn from(permissions: &PermissionsOverwrites<R>) -> Self {
        let allowed_permissions: Vec<String> = permissions
            .allow
            .items()
            .iter()
            .map(|item| item.to_string())
            .collect();

        let denied_permissions: Vec<String> = permissions
            .deny
            .items()
            .iter()
            .map(|item| item.to_string())
            .collect();

        Self {
            role: permissions.role.name(),
            allow: allowed_permissions,
            deny: denied_permissions,
        }
    }
}
