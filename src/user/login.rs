use axum::{debug_handler, http::StatusCode, response::IntoResponse, Extension, Json};
use entity::user;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use validator::Validate;

use crate::util::jwt::create_jwt;

#[debug_handler]
pub async fn login_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<LoginUserRequest>,
) -> Result<Json<LoginUserResponse>, UserLoginError> {
    user_data.validate()?;

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(&user_data.email))
        .one(&db)
        .await
        .map_err(UserLoginError::DatabaseError)?;

    let user = user.ok_or(UserLoginError::AccountNotFound)?;

    if bcrypt::verify(&user_data.password, &user.password_hash)
        .map_err(UserLoginError::ErrorVerifyingPassword)?
    {
        let token = create_jwt(&user_data.email).map_err(|_| {
            UserLoginError::InternalServerError("failed while creating jwt token".to_owned())
        })?;

        return Ok(Json(LoginUserResponse {
            token: token,
            type_of: "Bearer".to_owned(),
        }));
    } else {
        Err(UserLoginError::InvalidCredentials)
    }
}

#[derive(Validate, Deserialize)]
pub struct LoginUserRequest {
    #[validate(email)]
    email: String,
    #[validate(length(min = 4))]
    password: String,
}

#[derive(Serialize)]
pub struct LoginUserResponse {
    token: String,
    type_of: String,
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
            Self::AccountNotFound => StatusCode::NOT_FOUND,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::InvalidCredentials => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = json!({
            "error": self.to_string(),
        });

        (status, Json(body)).into_response()
    }
}
