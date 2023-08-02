use std::collections::HashSet;

use crate::base::ListComparison;

pub trait Category: Clone {
    fn name(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq)]
pub struct CategoriesList<C>
where
    C: Category,
{
    items: Vec<C>,
}

impl<C: Category> CategoriesList<C> {
    pub fn find_by_name(&self, name: &str) -> Option<&C> {
        self.items.iter().find(|category| category.name() == name)
    }

    pub fn find_by_name_panic(&self, name: &str) -> &C {
        self.find_by_name(name)
            .unwrap_or_else(|| panic!("No category found with name {name}."))
    }

    pub fn to_list(&self) -> &Vec<C> {
        &self.items
    }

    pub fn compare_by_name<'a, C2: Category>(
        &'a self,
        other: &'a CategoriesList<C2>,
    ) -> ListComparison<&C, &C2> {
        let mut extra_self: Vec<&C> = Vec::new();
        let mut extra_other: Vec<&C2> = Vec::new();
        let mut same: Vec<(&C, &C2)> = Vec::new();

        for self_item in self.to_list() {
            match other.find_by_name(self_item.name()) {
                Some(other_item) => same.push((self_item, other_item)),
                None => extra_self.push(self_item),
            }
        }

        for other_item in other.to_list() {
            if self.find_by_name(other_item.name()).is_none() {
                extra_other.push(other_item)
            }
        }

        ListComparison {
            extra_self,
            extra_other,
            same,
        }
    }
}

impl<C: Category> From<Vec<C>> for CategoriesList<C> {
    fn from(categories: Vec<C>) -> Self {
        let mut category_names: HashSet<String> = HashSet::new();

        for category in categories.iter() {
            if category_names.contains(category.name()) {
                panic!("All categories must have unique names.");
            }

            category_names.insert(category.name().to_string());
        }

        Self { items: categories }
    }
}
