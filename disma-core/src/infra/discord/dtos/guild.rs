use serde::Deserialize;

use crate::domain::entities::guild::GuildSummary;

#[derive(Deserialize)]
pub struct GuildResponse {
    pub name: String,
    pub id: String,
}

impl Into<GuildSummary> for GuildResponse {
    fn into(self) -> GuildSummary {
        GuildSummary {
            name: self.name,
            id: self.id,
        }
    }
}
