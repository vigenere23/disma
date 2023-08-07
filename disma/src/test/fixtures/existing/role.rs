#[cfg(test)]
pub mod tests {
    use crate::{permission::PermissionsList, role::ExistingRole};

    pub struct ExistingRoleFixture {
        id: String,
        name: String,
        permissions: PermissionsList,
        color: Option<String>,
        is_mentionable: bool,
        show_in_sidebar: bool,
    }

    impl ExistingRoleFixture {
        pub fn new() -> Self {
            Self {
                id: "123".to_string(),
                name: "abc".to_string(),
                permissions: PermissionsList::from(Vec::new()),
                color: None,
                is_mentionable: false,
                show_in_sidebar: false,
            }
        }

        pub fn with_name(mut self, name: &str) -> Self {
            self.name = name.to_string();
            self
        }

        pub fn build(self) -> ExistingRole {
            ExistingRole {
                id: self.id,
                name: self.name,
                permissions: self.permissions,
                color: self.color,
                is_mentionable: self.is_mentionable,
                show_in_sidebar: self.show_in_sidebar,
            }
        }
    }
}
