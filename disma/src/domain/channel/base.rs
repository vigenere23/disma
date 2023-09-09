use std::fmt::Display;

use crate::core::diffs::{Diff, Differ};

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

    // TODO more tests
}
