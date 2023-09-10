use crate::{
    core::events::{Change, ChangeEntity, ChangeEvent, ChangeEventListener},
    guild::{ExistingGuild, GuildCommander},
    role::{AwaitingRole, ExistingRole},
};

use super::Command;

pub struct AddRole {
    role: AwaitingRole,
}

impl AddRole {
    pub fn new(role: AwaitingRole) -> Self {
        Self { role }
    }

    fn describe(&self) -> Change {
        Change::Create(ChangeEntity::Role, self.role.name.clone())
    }
}

impl Command for AddRole {
    fn execute(
        &self,
        commander: &dyn GuildCommander,
        event_listener: &dyn ChangeEventListener,
        existing_guild: &mut ExistingGuild,
    ) {
        let result = commander.add_role(&self.role);

        let event = match result {
            Ok(role) => {
                existing_guild.add_or_replace_role(role);
                ChangeEvent::Success(self.describe())
            }
            Err(message) => ChangeEvent::Error(self.describe(), message),
        };

        event_listener.handle(event);
    }
}

pub struct UpdateRole {
    existing_role: ExistingRole,
    awaiting_role: AwaitingRole,
}

impl UpdateRole {
    pub fn new(existing_role: ExistingRole, awaiting_role: AwaitingRole) -> Self {
        Self {
            existing_role,
            awaiting_role,
        }
    }

    fn describe(&self) -> Change {
        Change::Update(ChangeEntity::Role, self.existing_role.name.to_string())
    }
}

impl Command for UpdateRole {
    fn execute(
        &self,
        commander: &dyn GuildCommander,
        event_listener: &dyn ChangeEventListener,
        existing_guild: &mut ExistingGuild,
    ) {
        let result = commander.update_role(&self.existing_role.id, &self.awaiting_role);

        let event = match result {
            Ok(role) => {
                existing_guild.add_or_replace_role(role);
                ChangeEvent::Success(self.describe())
            }
            Err(message) => ChangeEvent::Error(self.describe(), message),
        };

        event_listener.handle(event);
    }
}

pub struct DeleteRole {
    role: ExistingRole,
}

impl DeleteRole {
    pub fn new(role: ExistingRole) -> Self {
        Self { role }
    }

    fn describe(&self) -> Change {
        Change::Delete(ChangeEntity::Role, self.role.name.to_string())
    }
}

impl Command for DeleteRole {
    fn execute(
        &self,
        commander: &dyn GuildCommander,
        event_listener: &dyn ChangeEventListener,
        existing_guild: &mut ExistingGuild,
    ) {
        let result = commander.delete_role(&self.role.id);

        let event = match result {
            Ok(()) => {
                existing_guild.remove_role(self.role.clone());
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
        core::{
            commands::Command,
            events::{Change, ChangeEntity, ChangeEvent, ChangeEventListenerMock},
        },
        guild::{ExistingGuild, GuildCommanderMock},
        tests::fixtures::{
            commands::{AddRoleFixture, DeleteRoleFixture, UpdateRoleFixture},
            existing::{ExistingGuildFixture, ExistingRoleFixture},
        },
    };

    const AN_ERROR_MESSAGE: &str = "Unexpected error";
    const A_ROLE_NAME: &str = "role abc";

    fn setup() -> (GuildCommanderMock, ChangeEventListenerMock, ExistingGuild) {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let existing_guild = ExistingGuildFixture::new().build();

        event_listener.when_handle(any()).will_return_default();

        return (commander, event_listener, existing_guild);
    }

    #[test]
    fn when_adding_role_should_add_role_with_commander() {
        let (commander, event_listener, mut existing_guild) = setup();
        commander
            .when_add_role(any())
            .will_return(Ok(ExistingRoleFixture::new().build()));

        let add_command = AddRoleFixture::new().build();
        add_command.execute(&commander, &event_listener, &mut existing_guild);

        commander.expect_add_role(eq(&add_command.role));
    }

    #[test]
    fn given_failing_commander_when_adding_role_should_notify_of_error() {
        let (commander, event_listener, mut existing_guild) = setup();
        commander
            .when_add_role(any())
            .will_return(Err(AN_ERROR_MESSAGE.to_string()));

        let add_command = AddRoleFixture::new().build();
        add_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Create(ChangeEntity::Role, add_command.role.name.to_string()),
            AN_ERROR_MESSAGE.to_string(),
        )));
    }

    #[test]
    fn given_succeeding_commander_when_adding_role_should_notify_of_success_and_add_existing_role()
    {
        let (commander, event_listener, mut existing_guild) = setup();
        let created_role = ExistingRoleFixture::new().build();
        commander
            .when_add_role(any())
            .will_return(Ok(created_role.clone()));

        let add_command = AddRoleFixture::new().build();
        add_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Create(
            ChangeEntity::Role,
            add_command.role.name.to_string(),
        ))));
        assert_eq!(existing_guild.roles().to_list(), vec![&created_role]);
    }

    #[test]
    fn when_updating_role_should_update_role_with_commander() {
        let (commander, event_listener, mut existing_guild) = setup();
        commander
            .when_update_role(any(), any())
            .will_return(Ok(ExistingRoleFixture::new().build()));

        let update_command = UpdateRoleFixture::new().build();
        update_command.execute(&commander, &event_listener, &mut existing_guild);

        commander.expect_update_role(
            eq(&update_command.existing_role.id),
            eq(&update_command.awaiting_role),
        );
    }

    #[test]
    fn given_failing_commander_when_updating_role_should_notify_of_error() {
        let (commander, event_listener, mut existing_guild) = setup();
        commander
            .when_update_role(any(), any())
            .will_return(Err(AN_ERROR_MESSAGE.to_string()));

        let update_command = UpdateRoleFixture::new().build();
        update_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Update(
                ChangeEntity::Role,
                update_command.awaiting_role.name.to_string(),
            ),
            AN_ERROR_MESSAGE.to_string(),
        )));
    }

    #[test]
    fn given_succeeding_commander_when_updating_role_should_notify_of_success_and_replace_existing_role(
    ) {
        let (commander, event_listener, mut existing_guild) = setup();
        let existing_role = ExistingRoleFixture::new().with_name(A_ROLE_NAME).build();
        let updated_role = ExistingRoleFixture::new().with_name(A_ROLE_NAME).build();
        existing_guild.add_or_replace_role(existing_role);
        commander
            .when_update_role(any(), any())
            .will_return(Ok(updated_role.clone()));

        let update_command = UpdateRoleFixture::new().build();
        update_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Update(
            ChangeEntity::Role,
            update_command.awaiting_role.name.to_string(),
        ))));
        assert_eq!(existing_guild.roles().to_list(), vec![&updated_role]);
    }

    #[test]
    fn when_deleting_role_should_delete_role_with_commander() {
        let (commander, event_listener, mut existing_guild) = setup();
        commander.when_delete_role(any()).will_return(Ok(()));

        let delete_command = DeleteRoleFixture::new().build();
        delete_command.execute(&commander, &event_listener, &mut existing_guild);

        commander.expect_delete_role(eq(&delete_command.role.id));
    }

    #[test]
    fn given_failing_commander_when_deleting_role_should_notify_of_error() {
        let (commander, event_listener, mut existing_guild) = setup();
        commander
            .when_delete_role(any())
            .will_return(Err(AN_ERROR_MESSAGE.to_string()));

        let delete_command = DeleteRoleFixture::new().build();
        delete_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Delete(ChangeEntity::Role, delete_command.role.name.to_string()),
            AN_ERROR_MESSAGE.to_string(),
        )));
    }

    #[test]
    fn given_succeeding_commander_when_deleting_role_should_notify_of_success_and_remove_existing_role(
    ) {
        let (commander, event_listener, mut existing_guild) = setup();
        commander.when_delete_role(any()).will_return(Ok(()));

        let delete_command = DeleteRoleFixture::new().build();
        existing_guild.add_or_replace_role(delete_command.role.clone());
        assert!(!existing_guild.roles().to_list().is_empty());

        delete_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Delete(
            ChangeEntity::Role,
            delete_command.role.name.to_string(),
        ))));
        assert!(existing_guild.roles().to_list().is_empty());
    }
}
