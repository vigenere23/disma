use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    permission::{PermissionsList, PermissionsOverwrites},
    role::{AwaitingRole, ExistingRole, RolesList},
};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PermissionOverwriteType {
    Role = 0,
    // Member = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionOverwritesDto {
    #[serde(rename = "id")]
    pub role_or_member_id: String,
    #[serde(rename = "type")]
    pub _type: PermissionOverwriteType,
    pub allow: String,
    pub deny: String,
}

impl PermissionOverwritesDto {
    pub fn from(
        overwrites: &PermissionsOverwrites<AwaitingRole>,
        roles: &RolesList<ExistingRole>,
    ) -> Self {
        let role = roles
            .find_by_name(&overwrites.role.name)
            .unwrap_or_else(|| panic!("No role found for name {}", &overwrites.role.name));

        Self {
            _type: PermissionOverwriteType::Role,
            role_or_member_id: role.id.clone(),
            allow: overwrites.allow.code(),
            deny: overwrites.deny.code(),
        }
    }

    pub fn into(&self, roles: &RolesList<ExistingRole>) -> PermissionsOverwrites<ExistingRole> {
        PermissionsOverwrites {
            role: roles.find_by_id(&self.role_or_member_id).clone(),
            allow: PermissionsList::from(self.allow.as_str()),
            deny: PermissionsList::from(self.deny.as_str()),
        }
    }
}
