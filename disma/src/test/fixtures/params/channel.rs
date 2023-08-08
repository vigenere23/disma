#[cfg(test)]
pub mod tests {
    use crate::params::channel::{
        ChannelParams, ChannelParamsChannelType, ChannelParamsPermissionsOverwritesStrategy,
    };

    pub struct ChannelParamsFixture {
        name: String,
        permissions_overwrites: ChannelParamsPermissionsOverwritesStrategy,
        _type: ChannelParamsChannelType,
        topic: Option<String>,
        category: Option<String>,
    }

    impl ChannelParamsFixture {
        pub fn new() -> Self {
            Self {
                name: "abc".to_string(),
                permissions_overwrites: ChannelParamsPermissionsOverwritesStrategy::Manual {
                    items: Vec::new(),
                },
                _type: ChannelParamsChannelType::TEXT,
                topic: None,
                category: None,
            }
        }

        pub fn with_name(mut self, name: &str) -> Self {
            self.name = name.to_string();
            self
        }

        pub fn with_topic(mut self, topic: &str) -> Self {
            self.topic = Some(topic.to_string());
            self
        }

        pub fn with_category(mut self, category: &str) -> Self {
            self.category = Some(category.to_string());
            self
        }

        pub fn build(self) -> ChannelParams {
            ChannelParams {
                name: self.name,
                permissions_overwrites: self.permissions_overwrites,
                _type: self._type,
                topic: self.topic,
                category: self.category,
            }
        }
    }
}
