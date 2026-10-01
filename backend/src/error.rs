use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Conflict(String),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            ApiError::Internal(msg) => {
                tracing::error!("Internal server error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal server error occurred".to_string())
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

// Automatically convert sqlx::Error into our ApiError
impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ApiError::NotFound("Requested resource was not found".to_string()),
            sqlx::Error::Database(db_err) => {
                if db_err.is_unique_violation() {
                    // Usually code 23505 in Postgres
                    ApiError::Conflict("A record with this information already exists".to_string())
                } else if db_err.is_foreign_key_violation() {
                    ApiError::BadRequest("Invalid reference provided".to_string())
                } else {
                    ApiError::Internal(db_err.to_string())
                }
            }
            _ => ApiError::Internal(err.to_string()),
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::Internal(err.to_string())
    }
}

impl From<(StatusCode, String)> for ApiError {
    fn from(err: (StatusCode, String)) -> Self {
        match err.0 {
            StatusCode::NOT_FOUND => ApiError::NotFound(err.1),
            StatusCode::BAD_REQUEST => ApiError::BadRequest(err.1),
            StatusCode::UNAUTHORIZED => ApiError::Unauthorized(err.1),
            StatusCode::CONFLICT => ApiError::Conflict(err.1),
            _ => ApiError::Internal(err.1),
        }
    }
}
