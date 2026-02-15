use axum::{
    http::StatusCode,
    response::{IntoResponse,Response},
    Json,
};

use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid user")]
    UserInvalid,
    #[error("Internal sever error: {0}")]
    InternalServerErr(#[from] anyhow::Error)
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status,error_message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            AppError::UserInvalid => (StatusCode::FORBIDDEN, "Invalid user"),
            AppError::InternalServerErr(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        };

        let body = Json(json!({
            "error": error_message,
            "details": self.to_string(),
        }));

        (status, body).into_response()
    }
}