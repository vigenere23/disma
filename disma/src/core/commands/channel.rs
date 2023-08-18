use crate::{
    category::{CategoriesList, ExistingCategory},
    channel::{AwaitingChannel, Channel, ExistingChannel},
    core::events::{Change, ChangeEntity, ChangeEvent, ChangeEventListener},
    guild::GuildCommander,
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
    fn execute(&self, commander: &dyn GuildCommander, event_listener: &dyn ChangeEventListener) {
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
    fn execute(&self, commander: &dyn GuildCommander, event_listener: &dyn ChangeEventListener) {
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
    fn execute(&self, commander: &dyn GuildCommander, event_listener: &dyn ChangeEventListener) {
        let result = commander.delete_category(&self.channel.id);

        let event = match result {
            Ok(()) => ChangeEvent::Success(self.describe()),
            Err(message) => ChangeEvent::Error(self.describe(), message),
        };

        event_listener.handle(event);
    }
}

#[cfg(test)]
pub mod tests {
    use mock_it::{any, eq};

    use crate::{
        category::CategoriesList,
        channel::Channel,
        core::{
            commands::Command,
            events::{Change, ChangeEntity, ChangeEvent, ChangeEventListenerMock},
        },
        guild::GuildCommanderMock,
        role::RolesList,
        tests::fixtures::awaiting::AwaitingChannelFixture,
    };

    use super::AddChannel;

    fn given_add_channel_command() -> AddChannel {
        AddChannel::new(
            AwaitingChannelFixture::new().build(),
            RolesList::new(),
            CategoriesList::new(),
        )
    }

    #[test]
    fn when_adding_channel_should_add_channel_with_commander() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let add_command = given_add_channel_command();

        commander
            .when_add_channel(any(), any(), any())
            .will_return(Ok(()));
        event_listener.when_handle(any()).will_return_default();

        add_command.execute(&commander, &event_listener);

        commander.expect_add_channel(
            eq(&add_command.channel),
            eq(&add_command.roles),
            eq(&add_command.categories),
        );
    }

    #[test]
    fn given_failing_commander_when_adding_channel_should_notify_of_error() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let add_command = given_add_channel_command();
        let error_message = String::from("Unexpected error");

        commander
            .when_add_channel(any(), any(), any())
            .will_return(Err(error_message.clone()));
        event_listener.when_handle(any()).will_return_default();

        add_command.execute(&commander, &event_listener);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Create(
                ChangeEntity::Channel,
                add_command.channel.unique_name().to_string(),
            ),
            error_message,
        )));
    }

    #[test]
    fn given_succeeding_commander_when_adding_channel_should_notify_of_success() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let add_command = given_add_channel_command();

        commander
            .when_add_channel(any(), any(), any())
            .will_return(Ok(()));
        event_listener.when_handle(any()).will_return_default();

        add_command.execute(&commander, &event_listener);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Create(
            ChangeEntity::Channel,
            add_command.channel.unique_name().to_string(),
        ))));
    }
}
