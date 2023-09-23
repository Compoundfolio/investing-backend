use std::sync::Arc;

use axum::{
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    TypedHeader,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ApplicationState;

use super::model::{AuthenticationError, AppUser};


#[derive(Serialize, Deserialize)]
pub struct AuthClaims {
    pub iss: String,
    pub sub: Uuid,
    pub exp: usize,
    pub email: String,
}

#[axum::async_trait]
impl axum::extract::FromRequestParts<Arc<ApplicationState>> for AuthClaims {
    type Rejection = AuthenticationError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<ApplicationState>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, &state).await?;
        let claims = verify_jwt(bearer.token(), &state.settings.auth.server_secret)?;
        Ok(claims)
    }
}

pub fn generate_jwt(
    app_user: AppUser,
    secret: &str,
) -> Result<(String, usize), jsonwebtoken::errors::Error> {
    let expiration_time = chrono::Utc::now() + chrono::Duration::weeks(1);
    let expires_at = expiration_time.timestamp() as usize;
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &AuthClaims {
            iss: "compounfolio.com".to_owned(),
            sub: app_user.id,
            exp: expires_at,
            email: app_user.email,
        },
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok((token, expires_at))
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<AuthClaims, jsonwebtoken::errors::Error> {
    let token_data = jsonwebtoken::decode::<AuthClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &jsonwebtoken::Validation::default(),
    )?;
    Ok(token_data.claims)
}
