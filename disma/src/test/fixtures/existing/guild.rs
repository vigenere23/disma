#[cfg(test)]
pub mod tests {
    use crate::{
        category::{CategoriesList, ExistingCategory},
        channel::{ChannelsList, ExistingChannel},
        guild::ExistingGuild,
        role::{ExistingRole, RolesList},
    };

    pub struct ExistingGuildFixture {
        roles: RolesList<ExistingRole>,
        categories: CategoriesList<ExistingCategory>,
        channels: ChannelsList<ExistingChannel>,
    }

    impl ExistingGuildFixture {
        pub fn new() -> Self {
            Self {
                roles: RolesList::from(Vec::new()),
                categories: CategoriesList::from(Vec::new()),
                channels: ChannelsList::from(Vec::new()),
            }
        }

        pub fn with_role(mut self, role: ExistingRole) -> Self {
            self.roles.add(role);
            self
        }

        pub fn build(self) -> ExistingGuild {
            ExistingGuild {
                roles: self.roles,
                categories: self.categories,
                channels: self.channels,
            }
        }
    }
}
