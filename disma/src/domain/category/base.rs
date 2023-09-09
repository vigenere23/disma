use std::collections::HashMap;

use crate::core::ListComparison;

use super::ExistingCategory;

pub trait Category: Clone {
    fn name(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq)]
pub struct CategoriesList<C>
where
    C: Category,
{
    categories_by_name: HashMap<String, C>,
}

impl<C: Category> CategoriesList<C> {
    pub fn new() -> Self {
        Self {
            categories_by_name: HashMap::new(),
        }
    }

    pub fn find_by_name(&self, name: &str) -> Option<&C> {
        self.categories_by_name.get(name)
    }

    pub fn add(&mut self, category: C) {
        if self.categories_by_name.contains_key(category.name()) {
            // TODO replace with Result
            panic!("All categories must have unique names.");
        }

        self.categories_by_name
            .insert(category.name().to_string(), category);
    }

    pub fn to_list(&self) -> Vec<&C> {
        self.categories_by_name.values().collect()
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

impl<C: Category> Default for CategoriesList<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Category> From<Vec<C>> for CategoriesList<C> {
    fn from(categories: Vec<C>) -> Self {
        let mut categories_list = CategoriesList::new();

        for category in categories.into_iter() {
            categories_list.add(category);
        }

        categories_list
    }
}

impl CategoriesList<ExistingCategory> {
    pub fn find_by_id(&self, id: &str) -> Option<&ExistingCategory> {
        self.to_list()
            .into_iter()
            .find(|category| category.id == id)
    }

    pub fn add_or_replace(&mut self, category: ExistingCategory) {
        self.categories_by_name
            .insert(category.name().to_string(), category);
    }

    pub fn remove(&mut self, category: ExistingCategory) {
        self.categories_by_name.remove(category.name());
    }
}

#[cfg(test)]
mod tests {
    use crate::{category::ExistingCategory, tests::fixtures::existing::ExistingCategoryFixture};

    use super::CategoriesList;

    const SOME_NAME: &str = "non-existant";
    const SOME_ID: &str = "non-existant";

    #[test]
    fn can_find_by_name() {
        let category = ExistingCategoryFixture::new().build();
        let list = CategoriesList::from(vec![category.clone()]);

        let found = list.find_by_name(&category.name);

        assert!(found.is_some());
        assert_eq!(found.unwrap().to_owned(), category);
    }

    #[test]
    fn given_non_existant_when_finding_by_name_should_return_none() {
        let list = CategoriesList::<ExistingCategory>::new();

        let found = list.find_by_name(SOME_NAME);

        assert!(found.is_none());
    }

    #[test]
    fn can_find_by_id() {
        let category = ExistingCategoryFixture::new().build();
        let list = CategoriesList::from(vec![category.clone()]);

        let found = list.find_by_id(&category.id);

        assert!(found.is_some());
        assert_eq!(found.unwrap().to_owned(), category);
    }

    #[test]
    fn given_non_existant_when_finding_by_id_should_return_none() {
        let list = CategoriesList::<ExistingCategory>::new();

        let found = list.find_by_id(SOME_ID);

        assert!(found.is_none());
    }

    #[test]
    fn can_add_category() {
        let category = ExistingCategoryFixture::new().build();
        let mut list = CategoriesList::<ExistingCategory>::new();

        list.add(category.clone());

        assert_eq!(list.to_list(), vec![&category]);
    }

    #[test]
    #[should_panic]
    fn given_category_with_same_name_already_in_list_when_adding_category_should_panics() {
        let category = ExistingCategoryFixture::new().with_name(SOME_NAME).build();
        let category_copy = ExistingCategoryFixture::new().with_name(SOME_NAME).build();
        let mut list = CategoriesList::<ExistingCategory>::from(vec![category]);

        list.add(category_copy);
    }

    #[test]
    fn given_category_not_in_list_when_adding_or_replacing_should_add() {
        let category = ExistingCategoryFixture::new().build();
        let mut list = CategoriesList::<ExistingCategory>::new();

        list.add_or_replace(category.clone());

        assert_eq!(list.to_list(), vec![&category]);
    }

    #[test]
    fn given_category_already_in_list_when_adding_or_replacing_should_replace_according_to_name() {
        // TODO should probably replace according to id?
        // If the name changes (like for invalid characters), it might cause problems?
        let category = ExistingCategoryFixture::new().with_name(SOME_NAME).build();
        let category_clone = ExistingCategoryFixture::new().with_name(SOME_NAME).build();
        let mut list = CategoriesList::<ExistingCategory>::from(vec![category]);

        list.add_or_replace(category_clone.clone());

        assert_eq!(list.to_list(), vec![&category_clone]);
    }

    #[test]
    fn can_remove_category() {
        let category = ExistingCategoryFixture::new().build();
        let mut list = CategoriesList::<ExistingCategory>::from(vec![category.clone()]);

        list.remove(category);

        assert_eq!(list.to_list(), Vec::<&ExistingCategory>::new());
    }

    #[test]
    fn given_category_not_in_list_when_removing_category_should_do_nothing() {
        let non_existant_category = ExistingCategoryFixture::new().build();
        let mut list = CategoriesList::<ExistingCategory>::new();

        list.remove(non_existant_category);
    }
}
