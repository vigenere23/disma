use crate::{
    core::commands::category::{AddCategory, DeleteCategory, UpdateCategory},
    role::RolesList,
    tests::fixtures::{awaiting::AwaitingCategoryFixture, existing::ExistingCategoryFixture},
};

pub struct AddCategoryFixture {}

impl AddCategoryFixture {
    pub fn new() -> Self {
        AddCategoryFixture {}
    }

    pub fn build(self) -> AddCategory {
        AddCategory::new(AwaitingCategoryFixture::new().build(), RolesList::new())
    }
}

pub struct UpdateCategoryFixture {}

impl UpdateCategoryFixture {
    pub fn new() -> Self {
        UpdateCategoryFixture {}
    }

    pub fn build(self) -> UpdateCategory {
        UpdateCategory::new(
            ExistingCategoryFixture::new().build(),
            AwaitingCategoryFixture::new().build(),
            RolesList::new(),
        )
    }
}

pub struct DeleteCategoryFixture {}

impl DeleteCategoryFixture {
    pub fn new() -> Self {
        DeleteCategoryFixture {}
    }

    pub fn build(self) -> DeleteCategory {
        DeleteCategory::new(ExistingCategoryFixture::new().build())
    }
}
