use crate::{category::ExistingCategory, permission::PermissionsOverwritesList};

pub struct ExistingCategoryFixture {
    id: String,
    name: String,
    overwrites: PermissionsOverwritesList,
}

impl ExistingCategoryFixture {
    pub fn new() -> Self {
        Self {
            id: "123".to_string(),
            name: "abc".to_string(),
            overwrites: PermissionsOverwritesList::from(Vec::new()),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn build(self) -> ExistingCategory {
        ExistingCategory {
            id: self.id,
            name: self.name,
            overwrites: self.overwrites,
        }
    }
}
