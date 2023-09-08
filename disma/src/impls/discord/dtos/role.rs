use serde::{Deserialize, Serialize};

use crate::{
    permission::PermissionsList,
    role::{AwaitingRole, ExistingRole},
};

#[derive(Debug, Serialize, PartialEq)]
pub struct RoleRequest {
    pub name: String,
    pub permissions: String,
    pub color: Option<u32>,
    pub hoist: bool,
    pub mentionable: bool,
}

impl From<&AwaitingRole> for RoleRequest {
    fn from(role: &AwaitingRole) -> Self {
        Self {
            name: role.name.clone(),
            permissions: role.permissions.code(),
            color: role
                .color
                .clone()
                .map(|color| u32::from_str_radix(&color, 16).unwrap()),
            hoist: role.show_in_sidebar,
            mentionable: role.is_mentionable,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
    pub permissions: String,
    pub color: u32,
    pub hoist: bool,
    pub mentionable: bool,
}

impl Into<ExistingRole> for RoleResponse {
    fn into(self) -> ExistingRole {
        let color = match self.color {
            0 => None,
            color => Some(format!("{:0>6}", format!("{color:x}"))),
        };

        ExistingRole {
            id: self.id,
            name: self.name,
            permissions: PermissionsList::from(self.permissions.as_str()),
            color,
            is_mentionable: self.mentionable,
            show_in_sidebar: self.hoist,
        }
    }
}

#[cfg(test)]
mod tests {
    mod request {
        use crate::{
            impls::discord::dtos::role::RoleRequest, permission::PermissionsList,
            role::AwaitingRole,
        };

        #[test]
        fn can_be_created_from_awaiting_role() {
            let role = AwaitingRole {
                name: "role a".to_string(),
                permissions: PermissionsList::from("335577088"),
                color: Some("fb364a".to_string()),
                is_mentionable: false,
                show_in_sidebar: true,
            };

            let expected_request = RoleRequest {
                name: "role a".to_string(),
                permissions: "335577088".to_string(),
                color: Some(16463434),
                hoist: true,
                mentionable: false,
            };

            let request = RoleRequest::from(&role);

            assert_eq!(request, expected_request)
        }
    }

    mod response {
        use crate::{
            impls::discord::dtos::role::RoleResponse, permission::PermissionsList,
            role::ExistingRole,
        };

        #[test]
        fn can_convert_into_existing_role() {
            let response = RoleResponse {
                id: "abc-123".to_string(),
                name: "role a".to_string(),
                permissions: "335577088".to_string(),
                color: 16463434,
                hoist: true,
                mentionable: false,
            };

            let expected_role = ExistingRole {
                id: "abc-123".to_string(),
                name: "role a".to_string(),
                permissions: PermissionsList::from("335577088"),
                color: Some("fb364a".to_string()),
                is_mentionable: false,
                show_in_sidebar: true,
            };

            let role: ExistingRole = response.into();

            assert_eq!(role, expected_role)
        }
    }
}
