use std::collections::HashSet;

use crate::{
    diff::base::{Diff, Differ},
    overwrites::PermissionsOverwritesList,
    utils::misc::IfThen,
};

use super::role::{AwaitingRole, ExistingRole};

pub trait Category: Clone {
    fn name(&self) -> String;
}

#[derive(Clone, Debug)]
pub struct AwaitingCategory {
    pub name: String,
    pub overwrites: PermissionsOverwritesList<AwaitingRole>,
    // pub channels: Vec<AwaitingChannel>,
}

impl Category for AwaitingCategory {
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ExistingCategory {
    pub id: String,
    pub name: String,
    pub overwrites: PermissionsOverwritesList<ExistingRole>,
}

impl ExistingCategory {
    pub fn diffs_with(&self, awaiting: &AwaitingCategory) -> Vec<Diff> {
        let mut all_diffs = vec![];

        self.overwrites.diffs_with(&awaiting.overwrites).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("overwrites".into(), diffs)),
        );

        all_diffs
    }
}

impl Category for ExistingCategory {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl PartialEq<ExistingCategory> for AwaitingCategory {
    fn eq(&self, other: &ExistingCategory) -> bool {
        self.name == other.name && self.overwrites == other.overwrites
    }
}

#[derive(Debug)]
pub struct CategoriesList<T>
where
    T: Category,
{
    categories: Vec<T>,
}

impl<T: Category> CategoriesList<T> {
    pub fn find_by_name(&self, name: &str) -> Option<&T> {
        self.categories
            .iter()
            .find(|category| category.name() == name)
    }

    pub fn items(&self) -> &Vec<T> {
        &self.categories
    }
}

impl<T: Category> From<Vec<T>> for CategoriesList<T> {
    fn from(categories: Vec<T>) -> Self {
        let mut category_names: HashSet<String> = HashSet::new();

        for category in categories.iter() {
            if category_names.contains(&category.name()) {
                panic!("All categories must have unique names.");
            }

            category_names.insert(category.name().clone());
        }

        Self { categories }
    }
}
