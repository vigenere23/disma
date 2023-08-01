use std::collections::HashSet;

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
