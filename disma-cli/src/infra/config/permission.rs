use disma::{
    overwrites::PermissionsOverwrites,
    permission::PermissionsList,
    role::{Role, RolesList},
    utils::vec::Compress,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PermissionsOverwritesConfig {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Vec<String>>,
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
            allow: PermissionsList::from(&self.allow.unwrap_or_default()),
            deny: PermissionsList::from(&self.deny.unwrap_or_default()),
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
            allow: allowed_permissions.compress(),
            deny: denied_permissions.compress(),
        }
    }
}
