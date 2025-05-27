use crate::{
    domain::{
        repositories::adventurers::AdventurersRepository,
        value_object::adventurer_model::RegisterAdventurerModel,
    },
    infrastructure::argon2,
};
use anyhow::Result;
use std::sync::Arc;

pub struct AdventurersUseCase<T>
where
    T: AdventurersRepository + Send + Sync,
{
    adventurers_repository: Arc<T>,
}

impl<T> AdventurersUseCase<T>
where
    T: AdventurersRepository + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T>) -> Self {
        Self {
            adventurers_repository,
        }
    }

    pub async fn register(
        &self,
        mut register_adventurer_model: RegisterAdventurerModel,
    ) -> Result<i32> {
        let hashed_password = argon2::hash(register_adventurer_model.password.clone())?;

        register_adventurer_model.password = hashed_password;

        let register_entity = register_adventurer_model.to_entity();

        let adventurers_id = self
            .adventurers_repository
            .register(register_entity)
            .await?;

        Ok(adventurers_id)
    }
}
