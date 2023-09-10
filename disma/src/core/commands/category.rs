use crate::{
    category::{AwaitingCategory, ExistingCategory},
    core::events::{Change, ChangeEntity, ChangeEvent, ChangeEventListener},
    guild::{ExistingGuild, GuildCommander},
};

use super::Command;

pub struct AddCategory {
    category: AwaitingCategory,
}

impl AddCategory {
    pub fn new(category: AwaitingCategory) -> Self {
        Self { category }
    }

    fn describe(&self) -> Change {
        Change::Create(ChangeEntity::Category, self.category.name.to_string())
    }
}

impl Command for AddCategory {
    fn execute(
        &self,
        commander: &dyn GuildCommander,
        event_listener: &dyn ChangeEventListener,
        existing_guild: &mut ExistingGuild,
    ) {
        let result = commander.add_category(&self.category, existing_guild.roles());

        let event = match result {
            Ok(category) => {
                existing_guild.add_or_replace_category(category);
                ChangeEvent::Success(self.describe())
            }
            Err(message) => ChangeEvent::Error(self.describe(), message),
        };

        event_listener.handle(event);
    }
}

pub struct UpdateCategory {
    existing_category: ExistingCategory,
    awaiting_category: AwaitingCategory,
}

impl UpdateCategory {
    pub fn new(existing_category: ExistingCategory, awaiting_category: AwaitingCategory) -> Self {
        Self {
            existing_category,
            awaiting_category,
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
    fn execute(
        &self,
        commander: &dyn GuildCommander,
        event_listener: &dyn ChangeEventListener,
        existing_guild: &mut ExistingGuild,
    ) {
        let result = commander.update_category(
            &self.existing_category.id,
            &self.awaiting_category,
            existing_guild.roles(),
        );

        let event = match result {
            Ok(category) => {
                existing_guild.add_or_replace_category(category);
                ChangeEvent::Success(self.describe())
            }
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
    fn execute(
        &self,
        commander: &dyn GuildCommander,
        event_listener: &dyn ChangeEventListener,
        existing_guild: &mut ExistingGuild,
    ) {
        let result = commander.delete_category(&self.category.id);

        let event = match result {
            Ok(()) => {
                existing_guild.remove_category(self.category.clone());
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
            commands::{AddCategoryFixture, DeleteCategoryFixture, UpdateCategoryFixture},
            existing::{ExistingCategoryFixture, ExistingGuildFixture},
        },
    };

    const AN_ERROR_MESSAGE: &str = "Unexpected error";
    const A_CATEGORY_NAME: &str = "category abc";

    fn setup() -> (GuildCommanderMock, ChangeEventListenerMock, ExistingGuild) {
        let commander = GuildCommanderMock::new();
        let event_listener = ChangeEventListenerMock::new();
        let existing_guild = ExistingGuildFixture::new().build();

        event_listener.when_handle(any()).will_return_default();

        return (commander, event_listener, existing_guild);
    }

    #[test]
    fn when_adding_category_should_add_category_with_commander() {
        let (commander, event_listener, mut existing_guild) = setup();
        commander
            .when_add_category(any(), any())
            .will_return(Ok(ExistingCategoryFixture::new().build()));

        let add_command = AddCategoryFixture::new().build();
        add_command.execute(&commander, &event_listener, &mut existing_guild);

        commander.expect_add_category(eq(&add_command.category), eq(existing_guild.roles()));
    }

    #[test]
    fn given_failing_commander_when_adding_category_should_notify_of_error() {
        let (commander, event_listener, mut existing_guild) = setup();
        commander
            .when_add_category(any(), any())
            .will_return(Err(AN_ERROR_MESSAGE.to_string()));

        let add_command = AddCategoryFixture::new().build();
        add_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Create(
                ChangeEntity::Category,
                add_command.category.name.to_string(),
            ),
            AN_ERROR_MESSAGE.to_string(),
        )));
    }

    #[test]
    fn given_succeeding_commander_when_adding_category_should_notify_of_success_and_add_existing_category(
    ) {
        let (commander, event_listener, mut existing_guild) = setup();
        let created_category = ExistingCategoryFixture::new().build();
        commander
            .when_add_category(any(), any())
            .will_return(Ok(created_category.clone()));

        let add_command = AddCategoryFixture::new().build();
        add_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Create(
            ChangeEntity::Category,
            add_command.category.name.to_string(),
        ))));
        assert_eq!(
            existing_guild.categories().to_list(),
            vec![&created_category]
        );
    }

    #[test]
    fn when_updating_category_should_update_category_with_commander() {
        let (commander, event_listener, mut existing_guild) = setup();
        commander
            .when_update_category(any(), any(), any())
            .will_return(Ok(ExistingCategoryFixture::new().build()));

        let update_command = UpdateCategoryFixture::new().build();
        update_command.execute(&commander, &event_listener, &mut existing_guild);

        commander.expect_update_category(
            eq(&update_command.existing_category.id),
            eq(&update_command.awaiting_category),
            eq(existing_guild.roles()),
        );
    }

    #[test]
    fn given_failing_commander_when_updating_category_should_notify_of_error() {
        let (commander, event_listener, mut existing_guild) = setup();
        commander
            .when_update_category(any(), any(), any())
            .will_return(Err(AN_ERROR_MESSAGE.to_string()));

        let update_command = UpdateCategoryFixture::new().build();
        update_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Update(
                ChangeEntity::Category,
                update_command.awaiting_category.name.to_string(),
            ),
            AN_ERROR_MESSAGE.to_string(),
        )));
    }

    #[test]
    fn given_succeeding_commander_when_updating_category_should_notify_of_success_and_replace_existing_category(
    ) {
        let (commander, event_listener, mut existing_guild) = setup();
        let existing_category = ExistingCategoryFixture::new()
            .with_name(A_CATEGORY_NAME)
            .build();
        let updated_category = ExistingCategoryFixture::new()
            .with_name(A_CATEGORY_NAME)
            .build();
        existing_guild.add_or_replace_category(existing_category);
        commander
            .when_update_category(any(), any(), any())
            .will_return(Ok(updated_category.clone()));

        let update_command = UpdateCategoryFixture::new().build();
        update_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Update(
            ChangeEntity::Category,
            update_command.awaiting_category.name.to_string(),
        ))));
        assert_eq!(
            existing_guild.categories().to_list(),
            vec![&updated_category]
        );
    }

    #[test]
    fn when_deleting_category_should_delete_category_with_commander() {
        let (commander, event_listener, mut existing_guild) = setup();
        commander.when_delete_category(any()).will_return(Ok(()));

        let delete_command = DeleteCategoryFixture::new().build();
        delete_command.execute(&commander, &event_listener, &mut existing_guild);

        commander.expect_delete_category(eq(&delete_command.category.id));
    }

    #[test]
    fn given_failing_commander_when_deleting_category_should_notify_of_error() {
        let (commander, event_listener, mut existing_guild) = setup();
        commander
            .when_delete_category(any())
            .will_return(Err(AN_ERROR_MESSAGE.to_string()));

        let delete_command = DeleteCategoryFixture::new().build();
        delete_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Error(
            Change::Delete(
                ChangeEntity::Category,
                delete_command.category.name.to_string(),
            ),
            AN_ERROR_MESSAGE.to_string(),
        )));
    }

    #[test]
    fn given_succeeding_commander_when_deleting_category_should_notify_of_success_and_remove_category(
    ) {
        let (commander, event_listener, mut existing_guild) = setup();
        commander.when_delete_category(any()).will_return(Ok(()));

        let delete_command = DeleteCategoryFixture::new().build();
        existing_guild.add_or_replace_category(delete_command.category.clone());
        assert!(!existing_guild.categories().to_list().is_empty());

        delete_command.execute(&commander, &event_listener, &mut existing_guild);

        event_listener.expect_handle(eq(ChangeEvent::Success(Change::Delete(
            ChangeEntity::Category,
            delete_command.category.name.to_string(),
        ))));
        assert!(existing_guild.categories().to_list().is_empty());
    }
}
