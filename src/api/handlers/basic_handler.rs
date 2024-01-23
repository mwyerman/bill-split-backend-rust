use axum::{
    Json,
    http::StatusCode,
    response::IntoResponse,
};

pub async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json("404 Not Found")).into_response()
}

pub async fn hello() -> String {
    "Hello, World!".into()
}
