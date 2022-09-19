use std::collections::HashSet;

use super::{
    permission::PermissionsList,
    role::{AwaitingRole, ExistingRole, Role},
};

pub trait Category: Clone {
    fn name(&self) -> String;
}

#[derive(Clone, Debug)]
pub struct AwaitingCategory {
    pub name: String,
    pub permissions_overwrites: Option<Vec<CategoryPermissionsOverwrites<AwaitingRole>>>,
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
    pub permissions_overwrites: Option<Vec<CategoryPermissionsOverwrites<ExistingRole>>>,
}

impl Category for ExistingCategory {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl PartialEq<ExistingCategory> for AwaitingCategory {
    fn eq(&self, other: &ExistingCategory) -> bool {
        if self.name != other.name {
            return false;
        }

        return match (&self.permissions_overwrites, &other.permissions_overwrites) {
            (None, None) => true,
            (Some(permissions), Some(other_permissions)) => {
                if permissions.len() != other_permissions.len() {
                    return false;
                }

                permissions
                    .clone()
                    .sort_by(|a, b| a.role.name.cmp(&b.role.name));
                other_permissions
                    .clone()
                    .sort_by(|a, b| a.role.name.cmp(&b.role.name));

                for (permission, other_permission) in
                    permissions.iter().zip(other_permissions.iter())
                {
                    if permission != other_permission {
                        return false;
                    }
                }

                return true;
            }
            _ => false,
        };
    }
}

#[derive(Debug, Clone)]
pub struct CategoryPermissionsOverwrites<T>
where
    T: Role,
{
    pub role: T,
    pub allow: PermissionsList,
    pub deny: PermissionsList,
}

impl<T, U> PartialEq<CategoryPermissionsOverwrites<T>> for CategoryPermissionsOverwrites<U>
where
    T: Role,
    U: Role,
{
    fn eq(&self, other: &CategoryPermissionsOverwrites<T>) -> bool {
        self.role.name() == other.role.name()
            && self.allow == other.allow
            && self.deny == other.deny
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
