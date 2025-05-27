use crate::{
    application::usecases::guild_commanders::GuildCommandersUseCase,
    domain::{
        repositories::guild_commanders::GuildCommandersRepository,
        value_object::guild_commander_model::RegisterGuildCommanderModel,
    },
    infrastructure::postgresql::{
        connection::PgPoolSquad, repositories::guild_commanders::GuildCommanderPostgres,
    },
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use std::sync::Arc;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let guild_commander_repository = GuildCommanderPostgres::new(db_pool);
    let guild_commander_use_case =
        GuildCommandersUseCase::new(Arc::new(guild_commander_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(guild_commander_use_case))
}

pub async fn register<T>(
    State(guild_commander_use_case): State<Arc<GuildCommandersUseCase<T>>>,
    Json(register_guild_commander_model): Json<RegisterGuildCommanderModel>,
) -> impl IntoResponse
where
    T: GuildCommandersRepository + Send + Sync,
{
    match guild_commander_use_case
        .register(register_guild_commander_model)
        .await
    {
        Ok(guild_commander_id) => (
            StatusCode::CREATED,
            format!(
                "Register guild commander id: {} successfully!",
                guild_commander_id
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
