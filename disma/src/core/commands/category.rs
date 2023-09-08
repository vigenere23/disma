use crate::{
    category::{AwaitingCategory, ExistingCategory},
    core::events::{Change, ChangeEntity, ChangeEvent, ChangeEventListener},
    guild::GuildCommander,
    role::{ExistingRole, RolesList},
};

use super::Command;

pub struct AddCategory {
    category: AwaitingCategory,
    roles: RolesList<ExistingRole>,
}

impl AddCategory {
    pub fn new(category: AwaitingCategory, roles: RolesList<ExistingRole>) -> Self {
        Self { category, roles }
    }

    fn describe(&self) -> Change {
        Change::Create(ChangeEntity::Category, self.category.name.to_string())
    }
}

impl Command for AddCategory {
    fn execute(&self, commander: &dyn GuildCommander, event_listener: &dyn ChangeEventListener) {
        let result = commander.add_category(&self.category, &self.roles);

        let event = match result {
            Ok(_) => ChangeEvent::Success(self.describe()),
            Err(message) => ChangeEvent::Error(self.describe(), message),
        };

        event_listener.handle(event);
    }
}

pub struct UpdateCategory {
    existing_category: ExistingCategory,
    awaiting_category: AwaitingCategory,
    roles: RolesList<ExistingRole>,
}

impl UpdateCategory {
    pub fn new(
        existing_category: ExistingCategory,
        awaiting_category: AwaitingCategory,
        roles: RolesList<ExistingRole>,
    ) -> Self {
        Self {
            existing_category,
            awaiting_category,
            roles,
        }
    }

    fn describe(&self) -> Change {
        Change::Update(
            ChangeEntity::Category,
            self.existing_category.name.to_string(),
        )
    }
}

impl Command for UpdateCategory {
    fn execute(&self, commander: &dyn GuildCommander, event_listener: &dyn ChangeEventListener) {
        let result = commander.update_category(
            &self.existing_category.id,
            &self.awaiting_category,
            &self.roles,
        );

        let event = match result {
            Ok(_) => ChangeEvent::Success(self.describe()),
            Err(message) => ChangeEvent::Error(self.describe(), message),
        };

        event_listener.handle(event);
    }
}

pub struct DeleteCategory {
    category: ExistingCategory,
}

impl DeleteCategory {
    pub fn new(category: ExistingCategory) -> Self {
        Self { category }
    }

    fn describe(&self) -> Change {
        Change::Delete(ChangeEntity::Channel, self.category.name.to_string())
    }
}

impl Command for DeleteCategory {
    fn execute(&self, commander: &dyn GuildCommander, event_listener: &dyn ChangeEventListener) {
        let result = commander.delete_category(&self.category.id);

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
            commands::{AddCategoryFixture, DeleteCategoryFixture, UpdateCategoryFixture},
            existing::ExistingCategoryFixture,
        },
    };

    const AN_ERROR_MESSAGE: &str = "Unexpected error";

    #[test]
    fn when_adding_category_should_add_category_with_commander() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let add_command = AddCategoryFixture::new().build();

        commander
            .when_add_category(any(), any())
            .will_return(Ok(ExistingCategoryFixture::new().build()));
        event_listener.when_handle(any()).will_return_default();

        add_command.execute(&commander, &event_listener);

        commander.expect_add_category(eq(&add_command.category), eq(&add_command.roles));
    }

    #[test]
    fn given_failing_commander_when_adding_category_should_notify_of_error() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let add_command = AddCategoryFixture::new().build();

        commander
            .when_add_category(any(), any())
            .will_return(Err(AN_ERROR_MESSAGE.to_string()));
        event_listener.when_handle(any()).will_return_default();

        add_command.execute(&commander, &event_listener);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Create(
                ChangeEntity::Category,
                add_command.category.name.to_string(),
            ),
            AN_ERROR_MESSAGE.to_string(),
        )));
    }

    #[test]
    fn given_succeeding_commander_when_adding_category_should_notify_of_success() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let add_command = AddCategoryFixture::new().build();

        commander
            .when_add_category(any(), any())
            .will_return(Ok(ExistingCategoryFixture::new().build()));
        event_listener.when_handle(any()).will_return_default();

        add_command.execute(&commander, &event_listener);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Create(
            ChangeEntity::Category,
            add_command.category.name.to_string(),
        ))));
    }

    #[test]
    fn when_updating_category_should_update_category_with_commander() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let update_command = UpdateCategoryFixture::new().build();

        commander
            .when_update_category(any(), any(), any())
            .will_return(Ok(ExistingCategoryFixture::new().build()));
        event_listener.when_handle(any()).will_return_default();

        update_command.execute(&commander, &event_listener);

        commander.expect_update_category(
            eq(&update_command.existing_category.id),
            eq(&update_command.awaiting_category),
            eq(&update_command.roles),
        );
    }

    #[test]
    fn given_failing_commander_when_updating_category_should_notify_of_error() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let update_command = UpdateCategoryFixture::new().build();

        commander
            .when_update_category(any(), any(), any())
            .will_return(Err(AN_ERROR_MESSAGE.to_string()));
        event_listener.when_handle(any()).will_return_default();

        update_command.execute(&commander, &event_listener);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Update(
                ChangeEntity::Category,
                update_command.awaiting_category.name.to_string(),
            ),
            AN_ERROR_MESSAGE.to_string(),
        )));
    }

    #[test]
    fn given_succeeding_commander_when_updating_category_should_notify_of_success() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let update_command = UpdateCategoryFixture::new().build();

        commander
            .when_update_category(any(), any(), any())
            .will_return(Ok(ExistingCategoryFixture::new().build()));
        event_listener.when_handle(any()).will_return_default();

        update_command.execute(&commander, &event_listener);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Update(
            ChangeEntity::Category,
            update_command.awaiting_category.name.to_string(),
        ))));
    }

    #[test]
    fn when_deleting_category_should_delete_category_with_commander() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let delete_command = DeleteCategoryFixture::new().build();

        commander.when_delete_category(any()).will_return(Ok(()));
        event_listener.when_handle(any()).will_return_default();

        delete_command.execute(&commander, &event_listener);

        commander.expect_delete_category(eq(&delete_command.category.id));
    }

    #[test]
    fn given_failing_commander_when_deleting_category_should_notify_of_error() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let delete_command = DeleteCategoryFixture::new().build();

        commander
            .when_delete_category(any())
            .will_return(Err(AN_ERROR_MESSAGE.to_string()));
        event_listener.when_handle(any()).will_return_default();

        delete_command.execute(&commander, &event_listener);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Delete(
                ChangeEntity::Category,
                delete_command.category.name.to_string(),
            ),
            AN_ERROR_MESSAGE.to_string(),
        )));
    }

    #[test]
    fn given_succeeding_commander_when_deleting_category_should_notify_of_success() {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let delete_command = DeleteCategoryFixture::new().build();

        commander.when_delete_category(any()).will_return(Ok(()));
        event_listener.when_handle(any()).will_return_default();

        delete_command.execute(&commander, &event_listener);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Delete(
            ChangeEntity::Category,
            delete_command.category.name.to_string(),
        ))));
    }
}
