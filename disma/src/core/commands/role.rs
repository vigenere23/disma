use crate::{
    category::{CategoriesList, ExistingCategory},
    channel::{AwaitingChannel, Channel, ExistingChannel},
    core::events::{Change, ChangeEntity, ChangeEvent, ChangeEventListenerRef},
    guild::GuildCommanderRef,
    role::{ExistingRole, RolesList},
};

use super::Command;

pub struct AddChannel {
    channel: AwaitingChannel,
    roles: RolesList<ExistingRole>,
    categories: CategoriesList<ExistingCategory>,
}

impl AddChannel {
    pub fn new(
        channel: AwaitingChannel,
        roles: RolesList<ExistingRole>,
        categories: CategoriesList<ExistingCategory>,
    ) -> Self {
        Self {
            channel,
            roles,
            categories,
        }
    }

    fn describe(&self) -> Change {
        Change::Create(
            ChangeEntity::Channel,
            self.channel.unique_name().to_string(),
        )
    }
}

impl Command for AddChannel {
    fn execute(&self, commander: &GuildCommanderRef, event_listener: &ChangeEventListenerRef) {
        let result = commander.add_channel(&self.channel, &self.roles, &self.categories);

        let event = match result {
            Ok(()) => ChangeEvent::Success(self.describe()),
            Err(message) => ChangeEvent::Error(self.describe(), message),
        };

        event_listener.handle(event);
    }
}

pub struct UpdateChannel {
    existing_channel: ExistingChannel,
    awaiting_channel: AwaitingChannel,
    roles: RolesList<ExistingRole>,
    categories: CategoriesList<ExistingCategory>,
}

impl UpdateChannel {
    pub fn new(
        existing_channel: ExistingChannel,
        awaiting_channel: AwaitingChannel,
        roles: RolesList<ExistingRole>,
        categories: CategoriesList<ExistingCategory>,
    ) -> Self {
        Self {
            existing_channel,
            awaiting_channel,
            roles,
            categories,
        }
    }

    fn describe(&self) -> Change {
        Change::Update(
            ChangeEntity::Channel,
            self.existing_channel.unique_name().to_string(),
        )
    }
}

impl Command for UpdateChannel {
    fn execute(&self, commander: &GuildCommanderRef, event_listener: &ChangeEventListenerRef) {
        let result = commander.update_channel(
            &self.existing_channel.id,
            &self.awaiting_channel,
            &self.roles,
            &self.categories,
        );

        let event = match result {
            Ok(()) => ChangeEvent::Success(self.describe()),
            Err(message) => ChangeEvent::Error(self.describe(), message),
        };

        event_listener.handle(event);
    }
}

pub struct DeleteChannel {
    channel: ExistingChannel,
}

impl DeleteChannel {
    pub fn new(channel: ExistingChannel) -> Self {
        Self { channel }
    }

    fn describe(&self) -> Change {
        Change::Delete(
            ChangeEntity::Channel,
            self.channel.unique_name().to_string(),
        )
    }
}

impl Command for DeleteChannel {
    fn execute(&self, commander: &GuildCommanderRef, event_listener: &ChangeEventListenerRef) {
        let result = commander.delete_category(&self.channel.id);

        let event = match result {
            Ok(()) => ChangeEvent::Success(self.describe()),
            Err(message) => ChangeEvent::Error(self.describe(), message),
        };

        event_listener.handle(event);
    }
}
