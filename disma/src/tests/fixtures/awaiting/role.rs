use crate::{permission::PermissionsList, role::AwaitingRole};

pub struct AwaitingRoleFixture {
    name: String,
    permissions: PermissionsList,
    color: Option<String>,
    is_mentionable: bool,
    show_in_sidebar: bool,
}

impl AwaitingRoleFixture {
    pub fn new() -> Self {
        Self {
            name: "abc".to_string(),
            permissions: PermissionsList::from(Vec::new()),
            color: None,
            is_mentionable: false,
            show_in_sidebar: false,
        }
    }

    pub fn build(self) -> AwaitingRole {
        AwaitingRole {
            name: self.name,
            permissions: self.permissions,
            color: self.color,
            is_mentionable: self.is_mentionable,
            show_in_sidebar: self.show_in_sidebar,
        }
    }
}
