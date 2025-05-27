use crate::{
    application::usecases::quest_viewing::QuestViewingUseCase,
    domain::{
        repositories::quest_viewing::QuestViewingRepository,
        value_object::board_checking_filter::BoardCheckingFilter,
    },
    infrastructure::postgresql::{
        connection::PgPoolSquad, repositories::quest_viewing::QuestViewingPostgres,
    },
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use std::sync::Arc;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let quest_viewing_repository = QuestViewingPostgres::new(db_pool);
    let quest_viewing_use_case = QuestViewingUseCase::new(Arc::new(quest_viewing_repository));

    Router::new()
        .route("/:quest_id", get(view_details))
        .route("/board-checking", get(board_checking))
        .with_state(Arc::new(quest_viewing_use_case))
}

pub async fn view_details<T>(
    State(quest_viewing_use_case): State<Arc<QuestViewingUseCase<T>>>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T: QuestViewingRepository + Send + Sync,
{
    match quest_viewing_use_case.view_details(quest_id).await {
        Ok(quest_model) => (StatusCode::OK, Json(quest_model)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Quest not found").into_response(),
    }
}

pub async fn board_checking<T>(
    State(quest_viewing_use_case): State<Arc<QuestViewingUseCase<T>>>,
    filter: Query<BoardCheckingFilter>,
) -> impl IntoResponse
where
    T: QuestViewingRepository + Send + Sync,
{
    match quest_viewing_use_case.board_checking(&filter).await {
        Ok(quests_model) => (StatusCode::OK, Json(quests_model)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Quest not found").into_response(),
    }
}
