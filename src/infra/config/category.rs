use serde::{Deserialize, Serialize};

use crate::domain::{
    category::{AwaitingCategory, CategoryRolePermissions, ExistingCategory},
    permission::PermissionsList,
    role::{AwaitingRole, Role, RolesList},
};

#[derive(Serialize, Deserialize)]
pub struct CategoryConfig {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<CategoryRolePermissionsConfig>>,
}

impl From<&ExistingCategory> for CategoryConfig {
    fn from(category: &ExistingCategory) -> Self {
        Self {
            name: category.name.clone(),
            permissions: category.permissions.as_ref().map(|permissions| {
                permissions
                    .iter()
                    .map(CategoryRolePermissionsConfig::from)
                    .collect()
            }),
        }
    }
}

impl CategoryConfig {
    pub fn into(self, roles: &RolesList<AwaitingRole>) -> AwaitingCategory {
        AwaitingCategory {
            name: self.name,
            permission_overwrites: self.permissions.map(|permissions| {
                permissions
                    .into_iter()
                    .map(|permission| CategoryRolePermissions {
                        role: roles
                            .find_by_name(&permission.role)
                            .unwrap_or_else(|| {
                                panic!("No role found with name {}", &permission.role)
                            })
                            .clone(),
                        allow: PermissionsList::from(&permission.allow),
                        deny: PermissionsList::from(&permission.deny),
                    })
                    .collect()
            }),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CategoryRolePermissionsConfig {
    pub role: String,
    pub allow: Vec<String>,
    pub deny: Vec<String>,
}

impl<T> From<&CategoryRolePermissions<T>> for CategoryRolePermissionsConfig
where
    T: Role,
{
    fn from(permissions: &CategoryRolePermissions<T>) -> Self {
        Self {
            role: permissions.role.name(),
            allow: permissions
                .allow
                .items()
                .iter()
                .map(|item| item.to_string())
                .collect(),
            deny: permissions
                .deny
                .items()
                .iter()
                .map(|item| item.to_string())
                .collect(),
        }
    }
}
