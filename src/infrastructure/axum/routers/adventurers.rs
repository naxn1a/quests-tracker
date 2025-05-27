use crate::{
    application::usecases::adventurers::AdventurersUseCase,
    domain::{
        repositories::adventurers::AdventurersRepository,
        value_object::adventurer_model::RegisterAdventurerModel,
    },
    infrastructure::postgresql::{
        connection::PgPoolSquad, repositories::adventurers::AdventurersPostgres,
    },
};
use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use std::sync::Arc;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurers_repository = AdventurersPostgres::new(db_pool);
    let adventures_use_case = AdventurersUseCase::new(Arc::new(adventurers_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(adventures_use_case))
}

pub async fn register<T>(
    State(adventures_use_case): State<Arc<AdventurersUseCase<T>>>,
    Json(register_adventurer_model): Json<RegisterAdventurerModel>,
) -> impl IntoResponse
where
    T: AdventurersRepository + Send + Sync,
{
    unimplemented!();
}
