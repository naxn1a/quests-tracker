use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::quests::AddQuestEntity;

#[async_trait]
#[automock]
pub trait QuestOpsRepository {
    async fn add(&self, add_quest_entity: AddQuestEntity) -> Result<i32>;
    async fn edit(&self, quest_id: i32, edit_quest_entity: AddQuestEntity) -> Result<i32>;
    async fn remove(&self, quest_id: i32, guild_commander_id: i32) -> Result<()>;
}
