use crate::{
    core::events::{Change, ChangeEntity, ChangeEvent, ChangeEventListener},
    guild::GuildCommander,
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
    fn execute(&self, commander: &dyn GuildCommander, event_listener: &dyn ChangeEventListener) {
        let result = commander.add_role(&self.role);

        let event = match result {
            Ok(_) => ChangeEvent::Success(self.describe()),
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
    fn execute(&self, commander: &dyn GuildCommander, event_listener: &dyn ChangeEventListener) {
        let result = commander.update_role(&self.existing_role.id, &self.awaiting_role);

        let event = match result {
            Ok(_) => ChangeEvent::Success(self.describe()),
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
    fn execute(&self, commander: &dyn GuildCommander, event_listener: &dyn ChangeEventListener) {
        let result = commander.delete_role(&self.role.id);

        let event = match result {
            Ok(()) => ChangeEvent::Success(self.describe()),
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
        guild::GuildCommanderMock,
        tests::fixtures::{
            commands::{AddRoleFixture, DeleteRoleFixture, UpdateRoleFixture},
            existing::ExistingRoleFixture,
        },
    };

    const AN_ERROR_MESSAGE: &str = "Unexpected error";

    #[test]
    fn when_adding_role_should_add_role_with_commander() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let add_command = AddRoleFixture::new().build();

        commander
            .when_add_role(any())
            .will_return(Ok(ExistingRoleFixture::new().build()));
        event_listener.when_handle(any()).will_return_default();

        add_command.execute(&commander, &event_listener);

        commander.expect_add_role(eq(&add_command.role));
    }

    #[test]
    fn given_failing_commander_when_adding_role_should_notify_of_error() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let add_command = AddRoleFixture::new().build();

        commander
            .when_add_role(any())
            .will_return(Err(AN_ERROR_MESSAGE.to_string()));
        event_listener.when_handle(any()).will_return_default();

        add_command.execute(&commander, &event_listener);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Create(ChangeEntity::Role, add_command.role.name.to_string()),
            AN_ERROR_MESSAGE.to_string(),
        )));
    }

    #[test]
    fn given_succeeding_commander_when_adding_role_should_notify_of_success() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let add_command = AddRoleFixture::new().build();

        commander
            .when_add_role(any())
            .will_return(Ok(ExistingRoleFixture::new().build()));
        event_listener.when_handle(any()).will_return_default();

        add_command.execute(&commander, &event_listener);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Create(
            ChangeEntity::Role,
            add_command.role.name.to_string(),
        ))));
    }

    #[test]
    fn when_updating_role_should_update_role_with_commander() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let update_command = UpdateRoleFixture::new().build();

        commander
            .when_update_role(any(), any())
            .will_return(Ok(ExistingRoleFixture::new().build()));
        event_listener.when_handle(any()).will_return_default();

        update_command.execute(&commander, &event_listener);

        commander.expect_update_role(
            eq(&update_command.existing_role.id),
            eq(&update_command.awaiting_role),
        );
    }

    #[test]
    fn given_failing_commander_when_updating_role_should_notify_of_error() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let update_command = UpdateRoleFixture::new().build();

        commander
            .when_update_role(any(), any())
            .will_return(Err(AN_ERROR_MESSAGE.to_string()));
        event_listener.when_handle(any()).will_return_default();

        update_command.execute(&commander, &event_listener);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Update(
                ChangeEntity::Role,
                update_command.awaiting_role.name.to_string(),
            ),
            AN_ERROR_MESSAGE.to_string(),
        )));
    }

    #[test]
    fn given_succeeding_commander_when_updating_role_should_notify_of_success() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let update_command = UpdateRoleFixture::new().build();

        commander
            .when_update_role(any(), any())
            .will_return(Ok(ExistingRoleFixture::new().build()));
        event_listener.when_handle(any()).will_return_default();

        update_command.execute(&commander, &event_listener);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Update(
            ChangeEntity::Role,
            update_command.awaiting_role.name.to_string(),
        ))));
    }

    #[test]
    fn when_deleting_role_should_delete_role_with_commander() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let delete_command = DeleteRoleFixture::new().build();

        commander.when_delete_role(any()).will_return(Ok(()));
        event_listener.when_handle(any()).will_return_default();

        delete_command.execute(&commander, &event_listener);

        commander.expect_delete_role(eq(&delete_command.role.id));
    }

    #[test]
    fn given_failing_commander_when_deleting_role_should_notify_of_error() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let delete_command = DeleteRoleFixture::new().build();

        commander
            .when_delete_role(any())
            .will_return(Err(AN_ERROR_MESSAGE.to_string()));
        event_listener.when_handle(any()).will_return_default();

        delete_command.execute(&commander, &event_listener);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Delete(ChangeEntity::Role, delete_command.role.name.to_string()),
            AN_ERROR_MESSAGE.to_string(),
        )));
    }

    #[test]
    fn given_succeeding_commander_when_deleting_role_should_notify_of_success() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let delete_command = DeleteRoleFixture::new().build();

        commander.when_delete_role(any()).will_return(Ok(()));
        event_listener.when_handle(any()).will_return_default();

        delete_command.execute(&commander, &event_listener);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Delete(
            ChangeEntity::Role,
            delete_command.role.name.to_string(),
        ))));
    }
}
