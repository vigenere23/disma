use std::{collections::HashMap, fmt::Display};

use crate::core::{
    diffs::{Diff, Differ},
    ListComparison,
};

use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, PartialEq, Clone)]
pub enum ChannelType {
    TEXT,
    VOICE,
}

impl Differ<ChannelType> for ChannelType {
    fn diffs_with(&self, target: &ChannelType) -> Vec<Diff> {
        self.to_string().diffs_with(&target.to_string())
    }
}

#[derive(PartialEq, Debug)]
pub struct UniqueChannelName {
    channel_name: String,
    channel_type: ChannelType,
    category_name: String,
}

impl UniqueChannelName {
    pub fn from(
        channel_name: &str,
        channel_type: &ChannelType,
        category_name: Option<&str>,
    ) -> Self {
        Self {
            channel_name: channel_name.to_string(),
            channel_type: channel_type.clone(),
            category_name: category_name.unwrap_or_default().to_string(),
        }
    }
}

impl Display for UniqueChannelName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}:{} ({})",
            &self.category_name, &self.channel_name, &self.channel_type
        ))
    }
}

pub trait Channel {
    fn name(&self) -> &str;
    fn unique_name(&self) -> UniqueChannelName;
}

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

    pub fn push(&mut self, channel: C) {
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
            channels_list.push(channel);
        }

        channels_list
    }
}

#[cfg(test)]
mod tests {
    mod channel_type {
        use std::str::FromStr;

        use crate::channel::ChannelType;

        #[test]
        fn can_format_to_string() {
            let formatted = ChannelType::TEXT.to_string();
            assert_eq!(formatted, "TEXT");
        }

        #[test]
        fn can_be_parsed_from_string() {
            let parsed = ChannelType::from_str("TEXT").unwrap();
            assert_eq!(parsed, ChannelType::TEXT);
        }

        #[test]
        fn given_invalid_string_it_cannot_parse_from_string() {
            let parsed = ChannelType::from_str("bullshit");
            assert!(parsed.is_err());
        }
    }
}
