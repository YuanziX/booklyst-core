use axum::{http::StatusCode, Extension, Json};
use bcrypt::{hash, DEFAULT_COST};
use entity::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use validator::Validate;

use crate::{
    models::user::{CreateUserRequest, LoginUserRequest, UserCreationError, UserLoginError},
    util::jwt::create_jwt,
};

pub async fn create_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), UserCreationError> {
    user_data.validate()?;

    let hashed_password =
        hash(user_data.password, DEFAULT_COST).map_err(UserCreationError::PasswordHashingError)?;

    let user = user::ActiveModel {
        email: Set(user_data.email),
        password_hash: Set(hashed_password),
        first_name: Set(user_data.first_name),
        last_name: Set(user_data.last_name),
        ..Default::default()
    };

    user.insert(&db)
        .await
        .map_err(UserCreationError::DatabaseError)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "message": "Account created successfully"
        })),
    ))
}

pub async fn login_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<LoginUserRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), UserLoginError> {
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

        return Ok((
            StatusCode::ACCEPTED,
            Json(json!({
                "token": token,
                "type": "Bearer",
            })),
        ));
    } else {
        Err(UserLoginError::InvalidCredentials)
    }
}
