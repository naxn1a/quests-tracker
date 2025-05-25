use axum::{http::StatusCode, response::IntoResponse};

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found").into_response()
}

pub async fn check() -> impl IntoResponse {
    (StatusCode::OK, "Ok").into_response()
}
