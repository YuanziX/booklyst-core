use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use entity::book;
use sea_orm::{EntityTrait, ModelTrait};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use crate::AppState;

pub async fn delete_book(
    State(state): State<AppState>,
    Path(book_id): Path<i32>,
) -> Result<Json<DeleteBookResponse>, DeleteBookError> {
    book::Entity::find_by_id(book_id)
        .one(&state.db)
        .await
        .map_err(DeleteBookError::DatabaseError)?
        .ok_or_else(|| DeleteBookError::BookNotFound)?
        .delete(&state.db)
        .await
        .map_err(DeleteBookError::DatabaseError)?;

    Ok(Json(DeleteBookResponse {
        status: "OK".to_owned(),
        message: format!("Book {} deleted successfully", book_id),
    }))
}

#[derive(Serialize)]
pub struct DeleteBookResponse {
    status: String,
    message: String,
}

#[derive(Error, Debug)]
pub enum DeleteBookError {
    #[error("Book not found")]
    BookNotFound,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
}

impl IntoResponse for DeleteBookError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            &Self::BookNotFound => StatusCode::BAD_REQUEST,
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