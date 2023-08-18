use serde::Deserialize;

use crate::guild::GuildSummary;

#[derive(Deserialize)]
pub struct GuildResponse {
    pub name: String,
    pub id: String,
    pub approximate_member_count: u128,
}

impl Into<GuildSummary> for GuildResponse {
    fn into(self) -> GuildSummary {
        GuildSummary {
            name: self.name,
            id: self.id,
            nb_members: self.approximate_member_count,
        }
    }
}
