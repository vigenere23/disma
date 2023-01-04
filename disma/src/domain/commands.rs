use std::sync::Arc;

use crate::{
    domain::diff::Diff,
    guild::{ExistingGuild, GuildCommanderRef},
};

pub trait Command {
    fn execute(&self, guild_commander: &GuildCommanderRef);
    fn describe(&self) -> CommandDescription;
}
pub type CommandRef = Arc<dyn Command>;

pub trait CommandFactory {
    fn commands_for(&self, existing_guild: &ExistingGuild) -> Vec<CommandRef>;
}

pub enum CommandDescription {
    Create(CommandEntity, CommandEntityName),
    Delete(CommandEntity, CommandEntityName),
    Update(CommandEntity, CommandEntityName, Vec<Diff>),
}

type CommandEntityName = String;

#[derive(Debug)]
pub enum CommandEntity {
    Role,
    Category,
    Channel,
}

pub trait CommandEventListener {
    fn before_command_execution(&self, description: CommandDescription);
    fn after_command_execution(&self, description: CommandDescription);
}
pub type CommandEventListenerRef = Arc<dyn CommandEventListener>;

pub struct NullCommandEventListener {}

impl CommandEventListener for NullCommandEventListener {
    fn before_command_execution(&self, _: CommandDescription) {}
    fn after_command_execution(&self, _: CommandDescription) {}
}

#[cfg(test)]
mod tests {
    mod vec_diffs {
        use crate::diff::{Diff, Differ};

        #[test]
        fn it_calculates_additions() {
            let origin = vec!["hello"];
            let target = vec!["hello", "world!"];

            let diffs = origin.diffs_with(&target);

            let expected_diffs = vec![Diff::Add("world!".into())];
            assert_eq!(diffs, expected_diffs);
        }

        #[test]
        fn it_calculates_removals() {
            let origin = vec!["hello", "world!"];
            let target = vec!["hello"];

            let diffs = origin.diffs_with(&target);

            let expected_diffs = vec![Diff::Remove("world!".into())];
            assert_eq!(diffs, expected_diffs);
        }

        #[test]
        fn it_calculates_both_additions_and_removals() {
            let origin = vec!["super", "mario"];
            let target = vec!["hello", "world!"];

            let diffs = origin.diffs_with(&target);

            let expected_diffs = vec![
                Diff::Remove("super".into()),
                Diff::Remove("mario".into()),
                Diff::Add("hello".into()),
                Diff::Add("world!".into()),
            ];
            assert_eq!(diffs, expected_diffs);
        }

        #[test]
        fn given_same_arrays_returns_no_diff() {
            let origin = vec!["hello", "world!"];
            let target = vec!["hello", "world!"];

            let diffs = origin.diffs_with(&target);

            assert_eq!(diffs.len(), 0);
        }
    }

    mod str_diffs {
        use crate::diff::{Diff, Differ};

        #[test]
        fn given_same_str_returns_no_diff() {
            let origin = "hello";
            let target = "hello";

            let diffs = origin.diffs_with(&target);

            assert_eq!(diffs.len(), 0);
        }

        #[test]
        fn can_differ_strings() {
            let origin = "hello";
            let target = "world!";

            let diffs = origin.diffs_with(&target);

            let expected_diffs = vec![Diff::Remove("hello".into()), Diff::Add("world!".into())];
            assert_eq!(diffs, expected_diffs);
        }

        #[test]
        fn can_differ_bools() {
            let origin = true;
            let target = false;

            let diffs = origin.diffs_with(&target);

            let expected_diffs = vec![Diff::Remove("true".into()), Diff::Add("false".into())];
            assert_eq!(diffs, expected_diffs);
        }
    }
}
