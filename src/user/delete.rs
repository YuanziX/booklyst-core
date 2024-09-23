use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use entity::user;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

use crate::AppState;

pub async fn delete_user(
    Extension(email): Extension<String>,
    State(state): State<AppState>,
) -> Result<Json<DeleteUserResponse>, DeleteUserError> {
    user::Entity::find()
        .filter(user::Column::Email.eq(&email))
        .one(&state.db)
        .await
        .map_err(DeleteUserError::DatabaseError)?
        .ok_or_else(|| DeleteUserError::UserNotFound)?
        .delete(&state.db)
        .await
        .map_err(DeleteUserError::DatabaseError)?;

    Ok(Json(DeleteUserResponse {
        status: "OK".to_owned(),
        message: format!("user {} deleted successfully", email),
    }))
}

#[derive(Serialize)]
pub struct DeleteUserResponse {
    status: String,
    message: String,
}

#[derive(Error, Debug)]
pub enum DeleteUserError {
    #[error("user not found")]
    UserNotFound,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
}

impl IntoResponse for DeleteUserError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            &Self::UserNotFound => StatusCode::BAD_REQUEST,
            &Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (
            status,
            Json(json!(
                {
                    "error": &self.to_string()
                }
            )),
        )
            .into_response()
    }
}
