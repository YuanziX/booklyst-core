use axum::{routing::post, Router};

use super::login::login_user;
use super::register::register_user;

pub fn user_router() -> Router {
    let pub_router = Router::new()
        .route("/create", post(register_user))
        .route("/login", post(login_user));
    let auth_router = Router::new();

    Router::new().merge(pub_router).merge(auth_router)
}
