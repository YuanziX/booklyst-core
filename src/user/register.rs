use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, DEFAULT_COST};
use entity::user;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use validator::Validate;

use crate::AppState;

pub async fn register_user(
    State(state): State<AppState>,
    Json(user_data): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, UserCreationError> {
    user_data.validate()?;

    let hashed_password =
        hash(user_data.password, DEFAULT_COST).map_err(UserCreationError::PasswordHashingError)?;

    let user = user::ActiveModel {
        email: Set(user_data.email.clone()),
        password_hash: Set(hashed_password),
        first_name: Set(user_data.first_name),
        last_name: Set(user_data.last_name),
        ..Default::default()
    };

    user.insert(&state.db)
        .await
        .map_err(UserCreationError::DatabaseError)?;

    Ok(Json(CreateUserResponse {
        email: user_data.email,
        message: "account successfully created".to_owned(),
    }))
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    email: String,
    #[validate(length(min = 1))]
    first_name: String,
    #[validate(length(min = 1))]
    last_name: String,
    #[validate(length(min = 8, max = 32))]
    password: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    email: String,
    message: String,
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
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = json!({
            "error": self.to_string(),
        });

        (status, Json(body)).into_response()
    }
}
