use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    is_admin: bool,
    exp: usize,
}

pub fn create_jwt(
    email: &str,
    is_admin: bool,
    jwt_secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::weeks(4))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: email.to_owned(),
        is_admin: is_admin,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )?;

    Ok(token)
}

// returns email, is_admin
pub fn validate_jwt(
    token: &str,
    jwt_secret: &str,
) -> Result<JwtValidationResult, jsonwebtoken::errors::Error> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(JwtValidationResult {
        email: decoded.claims.sub,
        is_admin: decoded.claims.is_admin,
    })
}

pub struct JwtValidationResult {
    pub email: String,
    pub is_admin: bool,
}
