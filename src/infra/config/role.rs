use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::domain::{
    permission::{Permission, PermissionsList},
    role::{AwaitingRole, ExistingRole},
};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleConfig {
    Full(FullRoleConfig),
    Template(TemplateRoleConfig),
}

#[derive(Serialize, Deserialize)]
pub struct FullRoleConfig {
    pub name: String,
    pub permissions: Vec<String>,
    pub show_in_sidebar: bool,
    pub is_mentionable: bool,
}

impl From<&ExistingRole> for FullRoleConfig {
    fn from(role: &ExistingRole) -> Self {
        let permissions = role
            .permissions
            .items()
            .iter()
            .map(|permission| permission.to_string())
            .collect();

        Self {
            name: role.name.clone(),
            permissions,
            show_in_sidebar: role.show_in_sidebar,
            is_mentionable: role.is_mentionalbe,
        }
    }
}

impl From<&AwaitingRole> for FullRoleConfig {
    fn from(role: &AwaitingRole) -> Self {
        let permissions = role
            .permissions
            .items()
            .iter()
            .map(|permission| permission.to_string())
            .collect();

        Self {
            name: role.name.clone(),
            permissions,
            show_in_sidebar: role.show_in_sidebar,
            is_mentionable: role.is_mentionalbe,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TemplateRoleConfig {
    pub name: String,
    pub template: String,
}

pub struct RoleConfigAssembler {}

impl RoleConfigAssembler {
    pub fn to_awaiting(
        &self,
        role: &RoleConfig,
        templates: Option<&Vec<FullRoleConfig>>,
    ) -> AwaitingRole {
        match role {
            RoleConfig::Full(config) => self.using_full_config(config),
            RoleConfig::Template(config) => self.using_template(config, templates),
        }
    }

    fn using_full_config(&self, role_config: &FullRoleConfig) -> AwaitingRole {
        let permissions: Vec<Permission> = role_config
            .permissions
            .iter()
            .map(|permission| Permission::from_str(permission).unwrap())
            .collect();

        AwaitingRole {
            name: role_config.name.clone(),
            permissions: PermissionsList::from(&permissions),
            is_mentionalbe: role_config.is_mentionable,
            show_in_sidebar: role_config.show_in_sidebar,
        }
    }

    fn using_template(
        &self,
        config: &TemplateRoleConfig,
        templates: Option<&Vec<FullRoleConfig>>,
    ) -> AwaitingRole {
        if let Some(templates) = templates {
            let template = templates
                .iter()
                .find(|template| template.name == config.template);

            if let Some(template) = template {
                let config = FullRoleConfig {
                    name: config.name.clone(),
                    permissions: template.permissions.clone(),
                    is_mentionable: template.is_mentionable,
                    show_in_sidebar: template.show_in_sidebar,
                };
                self.using_full_config(&config)
            } else {
                panic!()
            }
        } else {
            panic!("No templates defined in config.")
        }
    }
}
