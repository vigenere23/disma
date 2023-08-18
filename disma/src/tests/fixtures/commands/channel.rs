use crate::{
    category::CategoriesList,
    core::commands::{AddChannel, DeleteChannel, UpdateChannel},
    role::RolesList,
    tests::fixtures::{awaiting::AwaitingChannelFixture, existing::ExistingChannelFixture},
};

pub struct AddChannelFixture {}

impl AddChannelFixture {
    pub fn new() -> Self {
        AddChannelFixture {}
    }

    pub fn build(self) -> AddChannel {
        AddChannel::new(
            AwaitingChannelFixture::new().build(),
            RolesList::new(),
            CategoriesList::new(),
        )
    }
}

pub struct UpdateChannelFixture {}

impl UpdateChannelFixture {
    pub fn new() -> Self {
        UpdateChannelFixture {}
    }

    pub fn build(self) -> UpdateChannel {
        UpdateChannel::new(
            ExistingChannelFixture::new().build(),
            AwaitingChannelFixture::new().build(),
            RolesList::new(),
            CategoriesList::new(),
        )
    }
}

pub struct DeleteChannelFixture {}

impl DeleteChannelFixture {
    pub fn new() -> Self {
        DeleteChannelFixture {}
    }

    pub fn build(self) -> DeleteChannel {
        DeleteChannel::new(ExistingChannelFixture::new().build())
    }
}
