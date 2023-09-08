use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    permission::{PermissionsList, PermissionsOverwrite},
    role::{ExistingRole, RolesList},
};

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum PermissionOverwriteType {
    Role = 0,
    // Member = 1,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PermissionOverwritesRequest {
    #[serde(rename = "id")]
    pub role_or_member_id: String,
    #[serde(rename = "type")]
    pub _type: PermissionOverwriteType,
    pub allow: String,
    pub deny: String,
}

impl PermissionOverwritesRequest {
    pub fn from(overwrites: &PermissionsOverwrite, roles: &RolesList<ExistingRole>) -> Self {
        let role = roles
            .find_by_name(&overwrites.name)
            .unwrap_or_else(|| panic!("No role found for name {}", &overwrites.name));

        Self {
            _type: PermissionOverwriteType::Role,
            role_or_member_id: role.id.clone(),
            allow: overwrites.allow.code(),
            deny: overwrites.deny.code(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionOverwritesResponse {
    #[serde(rename = "id")]
    pub role_or_member_id: String,
    #[serde(rename = "type")]
    pub _type: u8,
    pub allow: String,
    pub deny: String,
}

impl PermissionOverwritesResponse {
    pub fn _try_into(
        &self,
        roles: &RolesList<ExistingRole>,
    ) -> Result<PermissionsOverwrite, String> {
        if self._type != 0 {
            return Err(format!(
                "Unsupported permissions overwrite type {}",
                self._type
            ));
        };

        Ok(PermissionsOverwrite {
            name: roles
                .find_by_id(&self.role_or_member_id)
                // TODO should probably not panic, especially since it already returns a Result...
                .unwrap_or_else(|| {
                    panic!(
                        "Could not create permission overwrite from non-existing role with id '{}'",
                        &self.role_or_member_id
                    )
                })
                .name
                .clone(),
            allow: PermissionsList::from(self.allow.as_str()),
            deny: PermissionsList::from(self.deny.as_str()),
        })
    }
}

#[cfg(test)]
mod tests {
    mod request {
        use crate::{
            impls::discord::dtos::permissions::{
                PermissionOverwriteType, PermissionOverwritesRequest,
            },
            permission::{Permission, PermissionsList, PermissionsOverwrite},
            role::RolesList,
            tests::fixtures::existing::ExistingRoleFixture,
        };

        #[test]
        fn can_be_created_from_domain_entity() {
            let existing_role = ExistingRoleFixture::new().build();

            let permissions_overwrite = PermissionsOverwrite {
                name: existing_role.name.clone(),
                allow: PermissionsList::from(vec![
                    Permission::CREATE_INSTANT_INVITE,
                    Permission::VIEW_CHANNEL,
                ]),
                deny: PermissionsList::from(vec![Permission::EMBED_LINKS, Permission::SPEAK]),
            };

            let expected_request = PermissionOverwritesRequest {
                role_or_member_id: existing_role.id.clone(),
                _type: PermissionOverwriteType::Role,
                allow: "1025".to_string(),
                deny: "2113536".to_string(),
            };

            let request = PermissionOverwritesRequest::from(
                &permissions_overwrite,
                &RolesList::from(vec![existing_role]),
            );

            assert_eq!(request, expected_request);
        }

        #[test]
        #[should_panic]
        fn given_non_existant_role_when_creating_from_domain_entity_should_panic() {
            let permissions_overwrite = PermissionsOverwrite {
                name: "non-existant".to_string(),
                allow: PermissionsList::new(),
                deny: PermissionsList::new(),
            };

            PermissionOverwritesRequest::from(&permissions_overwrite, &RolesList::from(vec![]));
        }
    }

    mod response {
        use crate::{
            impls::discord::dtos::permissions::PermissionOverwritesResponse,
            permission::{Permission, PermissionsList, PermissionsOverwrite},
            role::RolesList,
            tests::fixtures::existing::ExistingRoleFixture,
        };

        #[test]
        fn can_be_converted_into_domain_entity() {
            let existing_role = ExistingRoleFixture::new().build();

            let response = PermissionOverwritesResponse {
                role_or_member_id: existing_role.id.clone(),
                _type: 0,
                allow: "1025".to_string(),
                deny: "2113536".to_string(),
            };

            let expected_entity = PermissionsOverwrite {
                name: existing_role.name.clone(),
                allow: PermissionsList::from(vec![
                    Permission::CREATE_INSTANT_INVITE,
                    Permission::VIEW_CHANNEL,
                ]),
                deny: PermissionsList::from(vec![Permission::EMBED_LINKS, Permission::SPEAK]),
            };

            let entity = response._try_into(&RolesList::from(vec![existing_role]));

            assert!(entity.is_ok());
            assert_eq!(entity.unwrap(), expected_entity);
        }

        #[test]
        #[should_panic]
        fn given_non_existant_role_when_converting_to_domain_entity_should_panic() {
            let response = PermissionOverwritesResponse {
                role_or_member_id: "non-existant id".to_string(),
                _type: 0,
                allow: "0".to_string(),
                deny: "0".to_string(),
            };

            response._try_into(&RolesList::from(vec![])).unwrap();
        }

        #[test]
        fn given_unsupported_type_when_converting_to_domain_entity_should_return_error() {
            let existing_role = ExistingRoleFixture::new().build();

            let response = PermissionOverwritesResponse {
                role_or_member_id: existing_role.id.clone(),
                _type: 2,
                allow: "0".to_string(),
                deny: "0".to_string(),
            };

            let entity = response._try_into(&RolesList::from(vec![existing_role]));

            assert!(entity.is_err());
        }
    }
}
