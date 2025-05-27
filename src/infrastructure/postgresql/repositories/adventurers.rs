use crate::{
    domain::{
        entities::adventurers::{AdventurerEntity, RegisterAdventurerEntity},
        repositories::adventurers::AdventurersRepository,
    },
    infrastructure::postgresql::connection::PgPoolSquad,
};
use anyhow::Result;
use axum::async_trait;
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
        unimplemented!();
    }

    async fn find_by_username(&self, username: String) -> Result<AdventurerEntity> {
        unimplemented!();
    }
}
