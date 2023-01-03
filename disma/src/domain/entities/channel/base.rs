use crate::diff::base::{Diff, Differ};

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

pub trait Channel {
    fn name(&self) -> String;
    fn category_name(&self) -> Option<String>;
    fn channel_type(&self) -> ChannelType;

    fn unique_name(&self) -> String {
        format!(
            "{}:{} ({})",
            &self.category_name().unwrap_or_default(),
            &self.name(),
            &self.channel_type().to_string()
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChannelsList<C>
where
    C: Channel,
{
    items: Vec<C>,
}

impl<C> ChannelsList<C>
where
    C: Channel,
{
    pub fn find(
        &self,
        name: &str,
        _type: ChannelType,
        category_name: Option<String>,
    ) -> Option<&C> {
        self.items.iter().find(|channel| {
            channel.name() == name
                && channel.channel_type() == _type
                && channel.category_name() == category_name
        })
    }

    pub fn to_list(&self) -> &Vec<C> {
        &self.items
    }
}

impl<C> From<Vec<C>> for ChannelsList<C>
where
    C: Channel,
{
    fn from(items: Vec<C>) -> Self {
        Self { items }
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
