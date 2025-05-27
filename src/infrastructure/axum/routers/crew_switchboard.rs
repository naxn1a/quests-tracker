use crate::{
    application::usecases::crew_switchboard::CrewSwitchboardUseCase,
    domain::repositories::{
        crew_switchboard::CrewSwitchboardRepository, quest_viewing::QuestViewingRepository,
    },
    infrastructure::postgresql::{
        connection::PgPoolSquad,
        repositories::{
            crew_switchboard::CrewSwitchboardPostgres, quest_viewing::QuestViewingPostgres,
        },
    },
};
use axum::{
    Extension, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, post},
};
use std::sync::Arc;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let crew_switchboard_repository = CrewSwitchboardPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let crew_switchboard_use_case = CrewSwitchboardUseCase::new(
        Arc::new(crew_switchboard_repository),
        Arc::new(quest_viewing_repository),
    );

    Router::new()
        .route("/join/:quest_id", post(join))
        .route("/leave/:quest_id", delete(leave))
        .with_state(Arc::new(crew_switchboard_use_case))
}

pub async fn join<T1, T2>(
    State(crew_switchboard_use_case): State<Arc<CrewSwitchboardUseCase<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewSwitchboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    (StatusCode::NOT_FOUND, "Unimplemented!").into_response() // << unimplemented!
}

pub async fn leave<T1, T2>(
    State(crew_switchboard_use_case): State<Arc<CrewSwitchboardUseCase<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewSwitchboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    (StatusCode::NOT_FOUND, "Unimplemented!").into_response() // << unimplemented!
}
