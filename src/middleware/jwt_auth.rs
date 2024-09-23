use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::user;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;
use thiserror::Error;

use crate::{
    util::{jwt::validate_jwt, models::RoleRequired},
    AppState,
};

pub async fn jwt_middleware(
    State(state): State<AppState>,
    Extension(role): Extension<RoleRequired>,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthenticationError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .ok_or_else(|| AuthenticationError::HeaderNotSupplied)?
        .to_str()
        .map_err(AuthenticationError::ReadingHeaderFailed)?;

    if !auth_header.starts_with("Bearer") {
        return Err(AuthenticationError::MalformedHeaderSupplied);
    }

    let token = &auth_header[7..];
    let jwt_validation_result = validate_jwt(token, &state.app_config.jwt_secret)
        .map_err(AuthenticationError::TokenNotValid)?;

    match (role, jwt_validation_result.is_admin) {
        (RoleRequired::Admin, false) => return Err(AuthenticationError::UserNotAllowed),
        (RoleRequired::User, true) => return Err(AuthenticationError::AdminNotAllowed),
        _ => {}
    }

    let _ = user::Entity::find()
        .filter(user::Column::Email.eq(&jwt_validation_result.email))
        .one(&state.db)
        .await
        .map_err(AuthenticationError::DatabaseError)?
        .ok_or_else(|| AuthenticationError::UserNotFound)?;

    request.extensions_mut().insert(jwt_validation_result.email);
    Ok(next.run(request).await)
}

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("user not allowed")]
    UserNotAllowed,
    #[error("admin not allowed")]
    AdminNotAllowed,
    #[error("authentication header not supplied")]
    HeaderNotSupplied,
    #[error("malformed header : header must start with 'Bearer'")]
    MalformedHeaderSupplied,
    #[error("user not found")]
    UserNotFound,
    #[error("database error : {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("failed to read the : {0}")]
    ReadingHeaderFailed(#[from] header::ToStrError),
    #[error("jwt token is not valid : {0}")]
    TokenNotValid(#[from] jsonwebtoken::errors::Error),
}

impl IntoResponse for AuthenticationError {
    fn into_response(self) -> Response {
        let status = match &self {
            Self::HeaderNotSupplied => StatusCode::BAD_REQUEST,
            Self::TokenNotValid(_) => StatusCode::BAD_REQUEST,
            Self::MalformedHeaderSupplied => StatusCode::BAD_REQUEST,
            Self::UserNotFound => StatusCode::BAD_REQUEST,
            Self::AdminNotAllowed => StatusCode::BAD_REQUEST,
            Self::UserNotAllowed => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = json!({
            "error": self.to_string(),
        });

        (status, Json(body)).into_response()
    }
}
