use crate::{
    application::usecases::quest_ops::QuestOpsUseCase,
    domain::{
        repositories::{quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository},
        value_object::quest_model::{AddQuestModel, EditQuestModel},
    },
    infrastructure::{
        axum::middlewares::guild_commanders_authorization,
        postgresql::{
            connection::PgPoolSquad,
            repositories::{quest_ops::QuestOpsPostgres, quest_viewing::QuestViewingPostgres},
        },
    },
};
use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, patch, post},
};
use std::sync::Arc;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let quest_ops_repository = QuestOpsPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let quest_ops_use_case = QuestOpsUseCase::new(
        Arc::new(quest_ops_repository),
        Arc::new(quest_viewing_repository),
    );

    Router::new()
        .route("/", post(add))
        .route("/:quest_id", patch(edit))
        .route("/:quest_id", delete(remove))
        .route_layer(middleware::from_fn(guild_commanders_authorization))
        .with_state(Arc::new(quest_ops_use_case))
}

pub async fn add<T1, T2>(
    State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Json(add_quest_model): Json<AddQuestModel>,
) -> impl IntoResponse
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match quest_ops_use_case
        .add(guild_commander_id, add_quest_model)
        .await
    {
        Ok(quest_id) => (
            StatusCode::CREATED,
            format!("Add quest success with id: {}", quest_id),
        )
            .into_response(),
        Err(err) => {
            tracing::error!("Failed to add quest: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to add quest").into_response()
        }
    }
}

pub async fn edit<T1, T2>(
    State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
    Json(edit_quest_model): Json<EditQuestModel>,
) -> impl IntoResponse
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match quest_ops_use_case
        .edit(quest_id, guild_commander_id, edit_quest_model)
        .await
    {
        Ok(updated_quest_id) => (
            StatusCode::OK,
            format!("Edit quest success with id: {}", updated_quest_id),
        )
            .into_response(),
        Err(err) => {
            tracing::error!("Failed to edit quest: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to edit quest").into_response()
        }
    }
}

pub async fn remove<T1, T2>(
    State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match quest_ops_use_case
        .remove(quest_id, guild_commander_id)
        .await
    {
        Ok(_) => (StatusCode::OK, "Quest removed successfully").into_response(),
        Err(err) => {
            tracing::error!("Failed to remove quest: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to remove quest").into_response()
        }
    }
}
