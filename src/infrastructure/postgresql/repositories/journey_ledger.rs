use crate::{
    domain::{
        repositories::journey_ledger::JourneyLedgerRepository,
        value_object::quest_statuses::QuestStatus,
    },
    infrastructure::postgresql::{connection::PgPoolSquad, schema::quests},
};
use anyhow::Result;
use axum::async_trait;
use diesel::{dsl::update, prelude::*};
use std::sync::Arc;

pub struct JourneyLedgerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl JourneyLedgerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl JourneyLedgerRepository for JourneyLedgerPostgres {
    async fn in_journey(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = update(quests::table)
            .filter(quests::id.eq(quest_id))
            .filter(quests::deleted_at.is_null())
            .set((
                quests::status.eq(QuestStatus::InJourney.to_string()),
                quests::guild_commander_id.eq(guild_commander_id),
            ))
            .returning(quests::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }

    async fn to_completed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = update(quests::table)
            .filter(quests::id.eq(quest_id))
            .filter(quests::deleted_at.is_null())
            .set((
                quests::status.eq(QuestStatus::Completed.to_string()),
                quests::guild_commander_id.eq(guild_commander_id),
            ))
            .returning(quests::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }

    async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = update(quests::table)
            .filter(quests::id.eq(quest_id))
            .filter(quests::deleted_at.is_null())
            .set((
                quests::status.eq(QuestStatus::Failed.to_string()),
                quests::guild_commander_id.eq(guild_commander_id),
            ))
            .returning(quests::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }
}
