use serde::{Deserialize, Serialize};

use crate::domain::{
    category::{CategoryRolePermissions, ExistingCategory},
    role::Role,
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

#[derive(Serialize, Deserialize)]
pub struct CategoryRolePermissionsConfig {
    pub role: String,
    pub allow: Vec<String>,
    pub deny: Vec<String>,
}

impl CategoryRolePermissionsConfig {
    pub fn from<T: Role>(permissions: &CategoryRolePermissions<T>) -> Self {
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
