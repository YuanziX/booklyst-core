use axum::routing::delete;
use axum::{middleware, Extension};
use axum::{routing::post, Router};

use super::delete::delete_user;
use super::login::login_user;
use super::register::register_user;
use crate::middleware::jwt_auth::jwt_middleware;
use crate::util::models::RoleRequired;
use crate::AppState;

pub fn user_router(state: &AppState) -> Router<AppState> {
    let pub_router = Router::new()
        .route("/create", post(register_user))
        .route("/login", post(login_user));

    let auth_router = Router::new()
        .route("/delete", delete(delete_user))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            jwt_middleware,
        ))
        .layer(Extension(RoleRequired::Either));

    Router::new().merge(pub_router).merge(auth_router)
}
