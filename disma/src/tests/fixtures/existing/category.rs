use fake::Fake;

use crate::{
    category::ExistingCategory,
    permission::{PermissionsOverwrite, PermissionsOverwritesList},
    role::ExistingRole,
};

pub struct ExistingCategoryFixture {
    id: String,
    name: String,
    overwrites: PermissionsOverwritesList<ExistingRole>,
}

impl ExistingCategoryFixture {
    pub fn new() -> Self {
        Self {
            id: fake::uuid::UUIDv4.fake(),
            name: fake::faker::lorem::en::Word().fake(),
            overwrites: PermissionsOverwritesList::from(Vec::new()),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_permissions_overwrites(
        mut self,
        overwrites: Vec<PermissionsOverwrite<ExistingRole>>,
    ) -> Self {
        self.overwrites = PermissionsOverwritesList::from(overwrites);
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
