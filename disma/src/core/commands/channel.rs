use crate::{
    channel::{AwaitingChannel, Channel, ExistingChannel},
    core::events::{Change, ChangeEntity, ChangeEvent, ChangeEventListener},
    guild::{ExistingGuild, GuildCommander},
};

use super::Command;

pub struct AddChannel {
    channel: AwaitingChannel,
}

impl AddChannel {
    pub fn new(channel: AwaitingChannel) -> Self {
        Self { channel }
    }

    fn describe(&self) -> Change {
        Change::Create(
            ChangeEntity::Channel,
            self.channel.unique_name().to_string(),
        )
    }
}

impl Command for AddChannel {
    fn execute(
        &self,
        commander: &dyn GuildCommander,
        event_listener: &dyn ChangeEventListener,
        existing_guild: &mut ExistingGuild,
    ) {
        let result = commander.add_channel(
            &self.channel,
            existing_guild.roles(),
            existing_guild.categories(),
        );

        let event = match result {
            Ok(channel) => {
                existing_guild.add_or_replace_channel(channel);
                ChangeEvent::Success(self.describe())
            }
            Err(message) => ChangeEvent::Error(self.describe(), message),
        };

        event_listener.handle(event);
    }
}

pub struct UpdateChannel {
    existing_channel: ExistingChannel,
    awaiting_channel: AwaitingChannel,
}

impl UpdateChannel {
    pub fn new(existing_channel: ExistingChannel, awaiting_channel: AwaitingChannel) -> Self {
        Self {
            existing_channel,
            awaiting_channel,
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
    fn execute(
        &self,
        commander: &dyn GuildCommander,
        event_listener: &dyn ChangeEventListener,
        existing_guild: &mut ExistingGuild,
    ) {
        let result = commander.update_channel(
            &self.existing_channel.id,
            &self.awaiting_channel,
            existing_guild.roles(),
            existing_guild.categories(),
        );

        let event = match result {
            Ok(channel) => {
                existing_guild.add_or_replace_channel(channel);
                ChangeEvent::Success(self.describe())
            }
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
    fn execute(
        &self,
        commander: &dyn GuildCommander,
        event_listener: &dyn ChangeEventListener,
        existing_guild: &mut ExistingGuild,
    ) {
        let result = commander.delete_channel(&self.channel.id);

        let event = match result {
            Ok(()) => {
                existing_guild.remove_channel(self.channel.clone());
                ChangeEvent::Success(self.describe())
            }
            Err(message) => ChangeEvent::Error(self.describe(), message),
        };

        event_listener.handle(event);
    }
}

#[cfg(test)]
mod tests {
    use mock_it::{any, eq};

    use crate::{
        channel::Channel,
        core::{
            commands::Command,
            events::{Change, ChangeEntity, ChangeEvent, ChangeEventListenerMock},
        },
        guild::GuildCommanderMock,
        tests::fixtures::{
            commands::{AddChannelFixture, DeleteChannelFixture, UpdateChannelFixture},
            existing::{ExistingChannelFixture, ExistingGuildFixture},
        },
    };

    const AN_ERROR_MESSAGE: &str = "Unexpected error";

    #[test]
    fn when_adding_channel_should_add_channel_with_commander() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let mut existing_guild = ExistingGuildFixture::new().build();
        let add_command = AddChannelFixture::new().build();

        commander
            .when_add_channel(any(), any(), any())
            .will_return(Ok(ExistingChannelFixture::new().build()));
        event_listener.when_handle(any()).will_return_default();

        add_command.execute(&commander, &event_listener, &mut existing_guild);

        commander.expect_add_channel(
            eq(&add_command.channel),
            eq(existing_guild.roles()),
            eq(existing_guild.categories()),
        );
    }

    #[test]
    fn given_failing_commander_when_adding_channel_should_notify_of_error() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let mut existing_guild = ExistingGuildFixture::new().build();
        let add_command = AddChannelFixture::new().build();

        commander
            .when_add_channel(any(), any(), any())
            .will_return(Err(AN_ERROR_MESSAGE.to_string()));
        event_listener.when_handle(any()).will_return_default();

        add_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Create(
                ChangeEntity::Channel,
                add_command.channel.unique_name().to_string(),
            ),
            AN_ERROR_MESSAGE.to_string(),
        )));
    }

    #[test]
    fn given_succeeding_commander_when_adding_channel_should_notify_of_success() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let mut existing_guild = ExistingGuildFixture::new().build();
        let add_command = AddChannelFixture::new().build();

        commander
            .when_add_channel(any(), any(), any())
            .will_return(Ok(ExistingChannelFixture::new().build()));
        event_listener.when_handle(any()).will_return_default();

        add_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Create(
            ChangeEntity::Channel,
            add_command.channel.unique_name().to_string(),
        ))));
    }

    #[test]
    fn when_updating_channel_should_update_channel_with_commander() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let mut existing_guild = ExistingGuildFixture::new().build();
        let update_command = UpdateChannelFixture::new().build();

        commander
            .when_update_channel(any(), any(), any(), any())
            .will_return(Ok(ExistingChannelFixture::new().build()));
        event_listener.when_handle(any()).will_return_default();

        update_command.execute(&commander, &event_listener, &mut existing_guild);

        commander.expect_update_channel(
            eq(&update_command.existing_channel.id),
            eq(&update_command.awaiting_channel),
            eq(existing_guild.roles()),
            eq(existing_guild.categories()),
        );
    }

    #[test]
    fn given_failing_commander_when_updating_channel_should_notify_of_error() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let mut existing_guild = ExistingGuildFixture::new().build();
        let update_command = UpdateChannelFixture::new().build();

        commander
            .when_update_channel(any(), any(), any(), any())
            .will_return(Err(AN_ERROR_MESSAGE.to_string()));
        event_listener.when_handle(any()).will_return_default();

        update_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Update(
                ChangeEntity::Channel,
                update_command.awaiting_channel.unique_name().to_string(),
            ),
            AN_ERROR_MESSAGE.to_string(),
        )));
    }

    #[test]
    fn given_succeeding_commander_when_updating_channel_should_notify_of_success() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let mut existing_guild = ExistingGuildFixture::new().build();
        let update_command = UpdateChannelFixture::new().build();

        commander
            .when_update_channel(any(), any(), any(), any())
            .will_return(Ok(ExistingChannelFixture::new().build()));
        event_listener.when_handle(any()).will_return_default();

        update_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Update(
            ChangeEntity::Channel,
            update_command.awaiting_channel.unique_name().to_string(),
        ))));
    }

    #[test]
    fn when_deleting_channel_should_delete_channel_with_commander() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let mut existing_guild = ExistingGuildFixture::new().build();
        let delete_command = DeleteChannelFixture::new().build();

        commander.when_delete_channel(any()).will_return(Ok(()));
        event_listener.when_handle(any()).will_return_default();

        delete_command.execute(&commander, &event_listener, &mut existing_guild);

        commander.expect_delete_channel(eq(&delete_command.channel.id));
    }

    #[test]
    fn given_failing_commander_when_deleting_channel_should_notify_of_error() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let mut existing_guild = ExistingGuildFixture::new().build();
        let delete_command = DeleteChannelFixture::new().build();

        commander
            .when_delete_channel(any())
            .will_return(Err(AN_ERROR_MESSAGE.to_string()));
        event_listener.when_handle(any()).will_return_default();

        delete_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Delete(
                ChangeEntity::Channel,
                delete_command.channel.unique_name().to_string(),
            ),
            AN_ERROR_MESSAGE.to_string(),
        )));
    }

    #[test]
    fn given_succeeding_commander_when_deleting_channel_should_notify_of_success() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let mut existing_guild = ExistingGuildFixture::new().build();
        let delete_command = DeleteChannelFixture::new().build();

        commander.when_delete_channel(any()).will_return(Ok(()));
        event_listener.when_handle(any()).will_return_default();

        delete_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Delete(
            ChangeEntity::Channel,
            delete_command.channel.unique_name().to_string(),
        ))));
    }
}
