use axum::{routing::get, Router};

use crate::handlers::misc_handler;

pub fn misc_router() -> Router {
    Router::new()
        .route("/", get(misc_handler::get_hello))
        .route("/health", get(misc_handler::get_health_handler))
}
