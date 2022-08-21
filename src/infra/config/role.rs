use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::domain::{
    permission::{Permission, PermissionsList},
    role::{AwaitingRole, ExistingRole},
};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleConfig {
    Full(RoleConfigFull),
    Template(RoleConfigUsingTemplate),
}

#[derive(Serialize, Deserialize)]
pub struct RoleConfigFull {
    pub name: String,
    pub permissions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub show_in_sidebar: bool,
    pub is_mentionable: bool,
}

impl From<&ExistingRole> for RoleConfigFull {
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
            color: role.color.clone(),
            show_in_sidebar: role.show_in_sidebar,
            is_mentionable: role.is_mentionalbe,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RoleConfigUsingTemplate {
    pub name: String,
    pub template: String,
    pub color: Option<String>,
    pub show_in_sidebar: Option<bool>,
    pub is_mentionable: Option<bool>,
}

pub struct RoleConfigAssembler {}

impl RoleConfigAssembler {
    pub fn to_awaiting(
        &self,
        role: &RoleConfig,
        templates: Option<&Vec<RoleConfigFull>>,
    ) -> AwaitingRole {
        match role {
            RoleConfig::Full(config) => self.using_full_config(config),
            RoleConfig::Template(config) => self.using_template(config, templates),
        }
    }

    fn using_full_config(&self, role_config: &RoleConfigFull) -> AwaitingRole {
        let permissions: Vec<Permission> = role_config
            .permissions
            .iter()
            .map(|permission| Permission::from_str(permission).unwrap())
            .collect();

        AwaitingRole {
            name: role_config.name.clone(),
            permissions: PermissionsList::from(&permissions),
            color: role_config.color.clone().map(|color| color.to_lowercase()),
            is_mentionable: role_config.is_mentionable,
            show_in_sidebar: role_config.show_in_sidebar,
        }
    }

    fn using_template(
        &self,
        config: &RoleConfigUsingTemplate,
        templates: Option<&Vec<RoleConfigFull>>,
    ) -> AwaitingRole {
        if let Some(templates) = templates {
            let template = templates
                .iter()
                .find(|template| template.name == config.template);

            if let Some(template) = template {
                let color = match config.color.clone() {
                    Some(color) => Some(color),
                    None => template.color.clone(),
                };

                let is_mentionable = match config.is_mentionable {
                    Some(is_mentionable) => is_mentionable,
                    None => template.is_mentionable,
                };

                let show_in_sidebar = match config.show_in_sidebar {
                    Some(show_in_sidebar) => show_in_sidebar,
                    None => template.show_in_sidebar,
                };

                let config = RoleConfigFull {
                    name: config.name.clone(),
                    permissions: template.permissions.clone(),
                    color,
                    is_mentionable,
                    show_in_sidebar,
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
