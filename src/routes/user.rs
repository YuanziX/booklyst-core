use crate::handlers::*;
use axum::{routing::post, Router};

pub fn user_router() -> Router {
    let pub_router = Router::new()
        .route("/create", post(user_handler::create_user))
        .route("/login", post(user_handler::login_user));
    let auth_router = Router::new();

    Router::new().merge(pub_router).merge(auth_router)
}
