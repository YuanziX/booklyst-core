use axum::{middleware, routing::{delete, get, post}, Extension, Router};

use crate::{middleware::jwt_auth::jwt_middleware, util::models::RoleRequired, AppState};

use super::{create::create_book, delete::delete_book, fetch::get_all_books};

pub fn book_router(state: &AppState) -> Router<AppState> {
    let pub_router = Router::new().route("/all/:page_no", get(get_all_books));
    
    // route for admin functions (add, delete, update books)
    let auth_router = Router::new()
    .route("/create", post(create_book))
    .route("/delete/:book_id", delete(delete_book))
    .route("/update", post(create_book))
    .layer(middleware::from_fn_with_state(
        state.clone(),
        jwt_middleware,
    )).layer(Extension(RoleRequired::Admin));

    Router::new()
    .merge(pub_router)
    .merge(auth_router)
}