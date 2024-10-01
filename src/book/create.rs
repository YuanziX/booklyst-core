use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use entity::book;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

use crate::AppState;

pub async fn create_book<'a>(
    State(state): State<AppState>,
    Json(create_book_request): Json<CreateBookRequest>,
) -> Result<Json<CreateBookResponse>, CreateBookError> {
    create_book_request.validate()?;

    // price is stored with accuracy up to 2 decimal places as an integer {supplied * 100}
    let book = book::ActiveModel {
        title: Set(create_book_request.title),
        author: Set(create_book_request.author),
        description: Set(create_book_request.description),
        price: Set((create_book_request.price * 100.0).trunc() as i32),
        available_stock: Set(create_book_request.available_stock as i32),
        is_rentable: Set(create_book_request.is_rentable),
        rental_price_per_day: Set((create_book_request.rental_price_per_day * 100.0).trunc() as i32),
        ..Default::default()
    }.insert(&state.db).await.map_err(CreateBookError::DatabaseError)?;

    Ok(Json(CreateBookResponse {
        status: "OK",
        message: format!("Book with id {} created successfully", &book.id).to_owned(),
    }))
}

#[derive(Validate, Deserialize)]
pub struct CreateBookRequest {
    #[validate(length(min = 1))]
    title: String,
    #[validate(length(min = 1))]
    author: String,
    #[validate(length(min = 1))]
    description: String,
    #[validate(range(min = 0.0))]
    price: f64,
    #[validate(range(min = 0))] 
    available_stock: u32,
    is_rentable: bool,
    #[validate(range(min = 0.0))]
    rental_price_per_day: f64,
}

#[derive(Debug, Error)]
pub enum CreateBookError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
}

impl IntoResponse for CreateBookError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            CreateBookError::ValidationError(_) => StatusCode::BAD_REQUEST,
            CreateBookError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}

#[derive(Serialize)]
pub struct CreateBookResponse {
    status: &'static str,
    message: String,
}