use std::fmt;
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

impl fmt::Display for DatabaseErrorDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Database error: {}", self.message)
    }
}

impl std::error::Error for DatabaseErrorDetails {}