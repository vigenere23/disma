use std::collections::HashMap;

use crate::core::ListComparison;

use super::{Channel, ExistingChannel, UniqueChannelName};

#[derive(Clone, Debug, PartialEq)]
pub struct ChannelsList<C>
where
    C: Channel,
{
    channels_by_name: HashMap<String, C>,
}

impl<C> ChannelsList<C>
where
    C: Channel,
{
    pub fn new() -> Self {
        Self {
            channels_by_name: HashMap::new(),
        }
    }

    pub fn find_by_unique_name(&self, unique_name: &UniqueChannelName) -> Option<&C> {
        self.channels_by_name.get(&unique_name.to_string())
    }

    pub fn add(&mut self, channel: C) {
        if self
            .channels_by_name
            .contains_key(&channel.unique_name().to_string())
        {
            // TODO replace with Result
            panic!("Channel '{}' already exists. All channels must have unique names and types within the same category.", channel.unique_name());
        }

        self.channels_by_name
            .insert(channel.unique_name().to_string(), channel);
    }

    pub fn to_list(&self) -> Vec<&C> {
        self.channels_by_name.values().collect()
    }

    pub fn compare_by_unique_name<'a, C2: Channel>(
        &'a self,
        other: &'a ChannelsList<C2>,
    ) -> ListComparison<&C, &C2> {
        let mut extra_self: Vec<&C> = Vec::new();
        let mut extra_other: Vec<&C2> = Vec::new();
        let mut same: Vec<(&C, &C2)> = Vec::new();

        for self_item in self.to_list() {
            match other.find_by_unique_name(&self_item.unique_name()) {
                Some(other_item) => same.push((self_item, other_item)),
                None => extra_self.push(self_item),
            }
        }

        for other_item in other.to_list() {
            if self
                .find_by_unique_name(&other_item.unique_name())
                .is_none()
            {
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

impl<C: Channel> Default for ChannelsList<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C> From<Vec<C>> for ChannelsList<C>
where
    C: Channel,
{
    fn from(items: Vec<C>) -> Self {
        let mut channels_list = ChannelsList::new();

        for channel in items.into_iter() {
            channels_list.add(channel);
        }

        channels_list
    }
}

impl ChannelsList<ExistingChannel> {
    pub fn add_or_replace(&mut self, channel: ExistingChannel) {
        self.channels_by_name
            .insert(channel.unique_name().to_string(), channel);
    }

    pub fn remove(&mut self, channel: ExistingChannel) {
        self.channels_by_name
            .remove(&channel.unique_name().to_string());
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        channel::{Channel, ChannelType, ExistingChannel, UniqueChannelName},
        core::ListComparison,
        tests::fixtures::existing::ExistingChannelFixture,
    };

    use super::ChannelsList;

    const SOME_NAME: &str = "non-existant";
    const SOME_TYPE: ChannelType = ChannelType::TEXT;

    #[test]
    fn can_find_by_unique_name() {
        let channel = ExistingChannelFixture::new().build();
        let list = ChannelsList::from(vec![channel.clone()]);

        let found = list.find_by_unique_name(&channel.unique_name());

        assert!(found.is_some());
        assert_eq!(found.unwrap().to_owned(), channel);
    }

    #[test]
    fn given_channel_not_in_list_when_finding_by_unique_name_should_return_none() {
        let list = ChannelsList::<ExistingChannel>::new();
        let non_existant_name = UniqueChannelName::from(SOME_NAME, &SOME_TYPE, None);

        let found = list.find_by_unique_name(&non_existant_name);

        assert!(found.is_none());
    }

    #[test]
    fn can_add_channel() {
        let channel = ExistingChannelFixture::new().build();
        let mut list = ChannelsList::<ExistingChannel>::new();

        list.add(channel.clone());

        assert_eq!(list.to_list(), vec![&channel]);
    }

    #[test]
    #[should_panic]
    fn given_channel_with_same_unique_name_already_in_list_when_adding_should_panics() {
        let channel = ExistingChannelFixture::new().with_name(SOME_NAME).build();
        let channel_copy = ExistingChannelFixture::new().with_name(SOME_NAME).build();
        let mut list = ChannelsList::from(vec![channel]);

        list.add(channel_copy);
    }

    #[test]
    fn given_channel_not_in_list_when_adding_or_replacing_should_add() {
        let channel = ExistingChannelFixture::new().build();
        let mut list = ChannelsList::<ExistingChannel>::new();

        list.add_or_replace(channel.clone());

        assert_eq!(list.to_list(), vec![&channel]);
    }

    #[test]
    fn given_channel_already_in_list_when_adding_or_replacing_should_replace_according_to_unique_name(
    ) {
        let channel = ExistingChannelFixture::new().with_name(SOME_NAME).build();
        let channel_clone = ExistingChannelFixture::new().with_name(SOME_NAME).build();
        let mut list = ChannelsList::from(vec![channel]);

        list.add_or_replace(channel_clone.clone());

        assert_eq!(list.to_list(), vec![&channel_clone]);
    }

    #[test]
    fn can_remove_channel() {
        let channel = ExistingChannelFixture::new().build();
        let mut list = ChannelsList::from(vec![channel.clone()]);

        list.remove(channel);

        assert_eq!(list.to_list(), Vec::<&ExistingChannel>::new());
    }

    #[test]
    fn given_channel_not_in_list_when_removing_should_do_nothing() {
        let non_existant_channel = ExistingChannelFixture::new().build();
        let mut list = ChannelsList::<ExistingChannel>::new();

        list.remove(non_existant_channel);
    }

    #[test]
    fn can_compare_lists_by_channel_unique_names() {
        let extra_self_channel = ExistingChannelFixture::new().build();
        let extra_other_channel = ExistingChannelFixture::new().build();
        let same_self_channel = ExistingChannelFixture::new().with_name(SOME_NAME).build();
        let same_other_channel = ExistingChannelFixture::new().with_name(SOME_NAME).build();

        let self_list =
            ChannelsList::from(vec![same_self_channel.clone(), extra_self_channel.clone()]);
        let other_list = ChannelsList::from(vec![
            same_other_channel.clone(),
            extra_other_channel.clone(),
        ]);

        let ListComparison {
            extra_self,
            extra_other,
            same,
        } = self_list.compare_by_unique_name(&other_list);

        assert_eq!(extra_self, vec![&extra_self_channel]);
        assert_eq!(extra_other, vec![&extra_other_channel]);
        assert_eq!(same, vec![(&same_self_channel, &same_other_channel)]);
    }
}
