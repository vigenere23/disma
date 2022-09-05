use std::collections::HashSet;

use super::{
    permission::PermissionsList,
    role::{AwaitingRole, ExistingRole, Role},
};

pub trait Category: Clone {
    fn name(&self) -> String;
}

#[derive(Clone)]
pub struct AwaitingCategory {
    pub name: String,
    pub permission_overwrites: Option<Vec<CategoryRolePermissions<AwaitingRole>>>,
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
    pub permissions: Option<Vec<CategoryRolePermissions<ExistingRole>>>,
}

impl Category for ExistingCategory {
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
pub struct CategoryRolePermissions<T>
where
    T: Role,
{
    pub role: T,
    pub allow: PermissionsList,
    pub deny: PermissionsList,
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
