use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Item not found")]
    NotFound,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Invalid input: {0}")]
    ValidationError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Item not found".to_string()),
            ApiError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg)
        };
        let body = Json(json!({
            "error": error_message
        }));
        (status, body).into_response()
    }
}