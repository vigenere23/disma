use crate::category::{AwaitingCategory, ExistingCategory};

use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, PartialEq)]
pub enum ChannelType {
    TEXT,
    VOICE,
}

#[derive(Debug, PartialEq)]
pub struct AwaitingChannel {
    pub name: String,
    pub topic: Option<String>,
    pub channel_type: ChannelType,
    pub category: Option<AwaitingCategory>,
    // pub overwrites: PermissionsOverwritesList<AwaitingRole>,
}

#[derive(Debug)]
pub struct ExistingChannel {
    pub id: String,
    pub name: String,
    pub topic: Option<String>,
    pub channel_type: ChannelType,
    pub category: Option<ExistingCategory>,
    // pub overwrites: PermissionsOverwritesList<ExistingRole>,
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
