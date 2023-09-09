use crate::{
    core::commands::role::{AddRole, DeleteRole, UpdateRole},
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
        let awaiting_role = AwaitingRoleFixture::new().build();
        UpdateRole::new(
            ExistingRoleFixture::new()
                .with_name(&awaiting_role.name)
                .build(),
            awaiting_role,
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
