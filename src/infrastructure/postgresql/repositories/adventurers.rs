use crate::{
    domain::{
        entities::adventurers::{AdventurerEntity, RegisterAdventurerEntity},
        repositories::adventurers::AdventurersRepository,
    },
    infrastructure::postgresql::{connection::PgPoolSquad, schema::adventurers},
};
use anyhow::{Ok, Result};
use axum::async_trait;
use diesel::{dsl::insert_into, prelude::*};
use std::sync::Arc;

pub struct AdventurersPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl AdventurersPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AdventurersRepository for AdventurersPostgres {
    async fn register(&self, register_adventurer_entity: RegisterAdventurerEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(adventurers::table)
            .values(register_adventurer_entity)
            .returning(adventurers::id)
            .get_result(&mut conn)?;

        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<AdventurerEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = adventurers::table
            .filter(adventurers::username.eq(username))
            .select(AdventurerEntity::as_select())
            .first(&mut conn)?;

        Ok(result)
    }
}
