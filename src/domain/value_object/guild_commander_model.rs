use crate::domain::entities::guild_commanders::RegisterGuildCommanderEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterGuildCommanderModel {
    pub username: String,
    pub password: String,
}

impl RegisterGuildCommanderModel {
    pub fn to_entity(self) -> RegisterGuildCommanderEntity {
        RegisterGuildCommanderEntity {
            username: self.username,
            password: self.password,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
