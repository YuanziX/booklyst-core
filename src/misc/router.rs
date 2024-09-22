use axum::{routing::get, Router};

pub fn misc_router() -> Router {
    Router::new()
        .route("/", get(super::hello::get_hello))
        .route("/health", get(super::health::health))
}
