use std::sync::Arc;

use crate::guild::GuildCommanderRef;

pub trait DiffCommand {
    fn execute(&self, guild_commander: &GuildCommanderRef);
    fn describe(&self) -> EntityChange;
}
pub type DiffCommandRef = Arc<dyn DiffCommand>;

pub enum EntityChange {
    Create(Entity, String),
    Delete(Entity, String),
    Update(Entity, String, Vec<Diff>),
}

#[derive(Debug)]
pub enum Entity {
    Role,
    Category,
    Channel,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Diff {
    Add(String),
    Remove(String),
    Update(String, Vec<Diff>),
}

pub trait Differ<T> {
    fn diffs_with(&self, target: &T) -> Vec<Diff>;
}

impl Differ<bool> for bool {
    fn diffs_with(&self, target: &Self) -> Vec<Diff> {
        diffs_between(self, target)
    }
}

impl Differ<String> for String {
    fn diffs_with(&self, target: &Self) -> Vec<Diff> {
        diffs_between(self, target)
    }
}

impl<'a, 'b> Differ<&'b str> for &'a str {
    fn diffs_with(&self, target: &&'b str) -> Vec<Diff> {
        diffs_between(self, target)
    }
}

impl<T, U> Differ<Option<U>> for Option<T>
where
    T: PartialEq<T> + ToString + Differ<U>,
    U: PartialEq<U> + ToString,
{
    fn diffs_with(&self, target: &Option<U>) -> Vec<Diff> {
        match (self, target) {
            (None, None) => vec![],
            (Some(origin), None) => vec![Diff::Remove(origin.to_string())],
            (None, Some(target)) => vec![Diff::Add(target.to_string())],
            (Some(origin), Some(target)) => origin.diffs_with(target),
        }
    }
}

impl<T> Differ<Vec<T>> for Vec<T>
where
    T: PartialEq<T> + ToString,
{
    fn diffs_with(&self, target: &Self) -> Vec<Diff> {
        let mut diffs = vec![];

        for item in self.iter() {
            if !target.contains(item) {
                diffs.push(Diff::Remove(item.to_string()))
            }
        }

        for item in target.iter() {
            if !self.contains(item) {
                diffs.push(Diff::Add(item.to_string()))
            }
        }

        diffs
    }
}

fn diffs_between<T>(origin: T, target: T) -> Vec<Diff>
where
    T: PartialEq<T> + ToString,
{
    let mut diffs = vec![];

    if origin != target {
        diffs.push(Diff::Remove(origin.to_string()));
        diffs.push(Diff::Add(target.to_string()));
    }

    diffs
}

#[cfg(test)]
mod tests {
    mod vec_diffs {
        use crate::diff::base::{Diff, Differ};

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
        use crate::diff::base::{Diff, Differ};

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
