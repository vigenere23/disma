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

#[cfg(test)]
mod tests {
    use crate::{
        permission::{PermissionsList, PermissionsOverwrite},
        role::{ExistingRole, RolesList},
        tests::fixtures::existing::ExistingRoleFixture,
    };

    use super::PermissionsOverwriteParams;

    const A_ROLE_NAME: &str = "role_a";

    #[test]
    fn can_convert_to_domain_entity() {
        let existing_roles = vec![ExistingRoleFixture::new().with_name(A_ROLE_NAME).build()];
        let params = PermissionsOverwriteParams {
            role: A_ROLE_NAME.to_string(),
            allow: vec![],
            deny: vec![],
        };

        let permissions_overwrite = params.into(&RolesList::<ExistingRole>::from(existing_roles));

        assert_eq!(
            permissions_overwrite,
            PermissionsOverwrite {
                name: A_ROLE_NAME.to_string(),
                allow: PermissionsList::new(),
                deny: PermissionsList::new()
            }
        )
    }

    #[test]
    #[should_panic]
    fn given_non_existant_role_when_converting_to_domain_entity_should_panic() {
        let params = PermissionsOverwriteParams {
            role: A_ROLE_NAME.to_string(),
            allow: vec![],
            deny: vec![],
        };

        params.into(&RolesList::<ExistingRole>::new());
    }

    #[test]
    fn can_create_from_domain_entity() {
        let permissions_overwrite = PermissionsOverwrite {
            name: A_ROLE_NAME.to_string(),
            allow: PermissionsList::new(),
            deny: PermissionsList::new(),
        };

        let params = PermissionsOverwriteParams::from(&permissions_overwrite);

        assert_eq!(
            params,
            PermissionsOverwriteParams {
                role: A_ROLE_NAME.to_string(),
                allow: vec![],
                deny: vec![],
            }
        )
    }
}
