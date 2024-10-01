use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use entity::book;
use sea_orm::{EntityTrait, QuerySelect};
use serde::Serialize;
use thiserror::Error;

use crate::AppState;

pub async fn get_all_books(
    State(state): State<AppState>,
    Path(page): Path<u64>,
) -> Result<Json<GetAllBooksResponse>, GetAllBooksError> {
    let offset: u64 = if page < 1 {
        return Err(GetAllBooksError::RequestError);
    } else {
        (page - 1) * 50 
    };

    let res = book::Entity::find().limit(50).offset(offset).all(&state.db).await.map_err(|e| GetAllBooksError::DatabaseError(e))?;

    Ok(Json(
        GetAllBooksResponse {
            books: res,
            page_no: page,
        }
    ))
}

#[derive(Serialize)]
pub struct GetAllBooksResponse {
    books: Vec<book::Model>,
    page_no: u64,
}

#[derive(Error, Debug)]
pub enum GetAllBooksError {
    #[error("malformed page no, expected number > 0")]
    RequestError,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
}

impl IntoResponse for GetAllBooksError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            GetAllBooksError::RequestError => StatusCode::BAD_REQUEST,
            GetAllBooksError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}