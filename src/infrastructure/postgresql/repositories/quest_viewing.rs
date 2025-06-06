use crate::{
    domain::{
        entities::quests::QuestEntity, repositories::quest_viewing::QuestViewingRepository,
        value_object::board_checking_filter::BoardCheckingFilter,
    },
    infrastructure::postgresql::{
        connection::PgPoolSquad,
        schema::{quest_adventurer_junction, quests},
    },
};
use anyhow::{Ok, Result};
use axum::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

pub struct QuestViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl QuestViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl QuestViewingRepository for QuestViewingPostgres {
    async fn view_details(&self, quest_id: i32) -> Result<QuestEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = quests::table
            .filter(quests::id.eq(quest_id))
            .filter(quests::deleted_at.is_null())
            .select(QuestEntity::as_select())
            .first::<QuestEntity>(&mut conn)?;

        Ok(result)
    }

    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let mut query = quests::table
            .filter(quests::deleted_at.is_null())
            .into_boxed();

        if let Some(name) = &filter.name {
            query = query.filter(quests::name.ilike(format!("%{}%", name)));
        }

        if let Some(status) = &filter.status {
            query = query.filter(quests::status.eq(status.to_string()));
        }

        let result = query
            .select(QuestEntity::as_select())
            .order_by(quests::created_at.desc())
            .load::<QuestEntity>(&mut conn)?;

        Ok(result)
    }

    async fn adventurers_counting_by_quest_id(&self, quest_id: i32) -> Result<i64> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = quest_adventurer_junction::table
            .filter(quest_adventurer_junction::quest_id.eq(quest_id))
            .count()
            .first::<i64>(&mut conn)?;

        Ok(result)
    }
}
