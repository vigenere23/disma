use crate::{api::params::role::RoleParams, permission::Permission};

pub struct RoleParamsFixture {
    name: String,
    permissions: Vec<Permission>,
    color: Option<String>,
    show_in_sidebar: bool,
    is_mentionable: bool,
}

impl RoleParamsFixture {
    pub fn new() -> Self {
        Self {
            name: "abc".to_string(),
            permissions: Vec::new(),
            color: None,
            show_in_sidebar: false,
            is_mentionable: false,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    pub fn build(self) -> RoleParams {
        RoleParams {
            name: self.name,
            permissions: self.permissions,
            color: self.color,
            show_in_sidebar: self.show_in_sidebar,
            is_mentionable: self.is_mentionable,
        }
    }
}
