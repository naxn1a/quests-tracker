use serde::{Deserialize, Serialize};

use crate::domain::entities::quests::{AddQuestEntity, EditQuestEntity};

use super::quest_statuses::QuestStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub guild_commander_id: i32,
    pub adventurers_count: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddQuestModel {
    pub name: String,
    pub description: Option<String>,
}

impl AddQuestModel {
    pub fn to_entity(&self, guild_commander_id: i32) -> AddQuestEntity {
        AddQuestEntity {
            name: self.name.clone(),
            description: self.description.clone(),
            status: QuestStatus::Open.to_string(),
            guild_commander_id,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditQuestModel {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl EditQuestModel {
    pub fn to_entity(&self, guild_commander_id: i32) -> EditQuestEntity {
        EditQuestEntity {
            name: self.name.clone(),
            description: self.description.clone(),
            guild_commander_id,
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
