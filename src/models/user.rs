use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    #[validate(length(min = 4))]
    pub password: String,
}

#[derive(Validate, Deserialize)]
pub struct LoginUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 4))]
    pub password: String,
}

#[derive(Error, Debug)]
pub enum UserCreationError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("Password hashing error: {0}")]
    PasswordHashingError(#[from] bcrypt::BcryptError),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
}

impl IntoResponse for UserCreationError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            UserCreationError::ValidationError(_) => StatusCode::BAD_REQUEST,
            UserCreationError::PasswordHashingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UserCreationError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = json!({
            "error": self.to_string(),
        });

        (status, Json(body)).into_response()
    }
}

#[derive(Error, Debug)]
pub enum UserLoginError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("error verifying password: {0}")]
    ErrorVerifyingPassword(#[from] bcrypt::BcryptError),
    #[error("Account not found")]
    AccountNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
}

impl IntoResponse for UserLoginError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::AccountNotFound => StatusCode::NOT_FOUND,
            Self::InvalidCredentials => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = json!({
            "error": self.to_string(),
        });

        (status, Json(body)).into_response()
    }
}
