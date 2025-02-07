use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;
use utoipa::ToSchema;
use sqlx::Error as SqlxError;

#[derive(Debug, ToSchema)]
pub struct DatabaseErrorDetails {
    message: String,
}

impl From<SqlxError> for DatabaseErrorDetails {
    fn from(err: SqlxError) -> Self {
        DatabaseErrorDetails {
            message: err.to_string(),
        }
    }
}


#[derive(Debug, Error, ToSchema)]
pub enum ApiError {
    #[error("Item not found")]
    NotFound,

    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseErrorDetails),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Item not found".to_string()),
            ApiError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
        };
        let body = Json(json!({
            "error": error_message
        }));
        (status, body).into_response()
    }
}