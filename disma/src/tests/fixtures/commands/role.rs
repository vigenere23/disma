use crate::{
    core::commands::{AddRole, DeleteRole, UpdateRole},
    tests::fixtures::{awaiting::AwaitingRoleFixture, existing::ExistingRoleFixture},
};

pub struct AddRoleFixture {}

impl AddRoleFixture {
    pub fn new() -> Self {
        AddRoleFixture {}
    }

    pub fn build(self) -> AddRole {
        AddRole::new(AwaitingRoleFixture::new().build())
    }
}

pub struct UpdateRoleFixture {}

impl UpdateRoleFixture {
    pub fn new() -> Self {
        UpdateRoleFixture {}
    }

    pub fn build(self) -> UpdateRole {
        UpdateRole::new(
            ExistingRoleFixture::new().build(),
            AwaitingRoleFixture::new().build(),
        )
    }
}

pub struct DeleteRoleFixture {}

impl DeleteRoleFixture {
    pub fn new() -> Self {
        DeleteRoleFixture {}
    }

    pub fn build(self) -> DeleteRole {
        DeleteRole::new(ExistingRoleFixture::new().build())
    }
}
