use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use redis::Commands;
use serde::Deserialize;
use std::sync::Arc;
use tracing::warn;
use validator::Validate;

use crate::datasource::diesel::model::auth::{AppUser, InsertAppUser, InsertLoginMethod};
use crate::datasource::diesel::{enums::LoginMethodType, repository::RepositoryError};
use crate::web::model::auth::*;
use crate::ApplicationState;

const STATE_TOKEN_REDIS_PREFIX: &str = "STATE_TOKEN";

pub fn routes() -> Router<Arc<ApplicationState>> {
    Router::new()
        .route("/auth/state", get(get_state_token))
        .route("/auth/signup", post(post_auth_signup))
        .route("/auth/signin", post(post_auth_signin))
        .route("/auth/google", post(post_auth_google))
}

async fn get_state_token(
    State(app_state): State<Arc<ApplicationState>>,
) -> Result<(StatusCode, Json<StateTokenResponse>), AuthenticationFlowError> {
    let state_token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let mut connection = app_state.redis.get_connection()?;
    let state_token_key: String = format!("{}:{}", STATE_TOKEN_REDIS_PREFIX, state_token);
    connection.set_ex(&state_token_key, "", 3600_usize)?;
    Ok((
        StatusCode::OK,
        Json(StateTokenResponse { state: state_token }),
    ))
}

async fn post_auth_signup(
    State(state): State<Arc<ApplicationState>>,
    Json(request): Json<SignupAuthRequest>,
) -> impl IntoResponse {
    request.validate()?;

    if state
        .repository
        .find_user_by_email(&request.email)?
        .is_some()
    {
        return Err(AuthenticationFlowError::UserAlreadyExists);
    }

    let salt_string = SaltString::from_b64(&state.settings.auth.password_salt)
        .map_err(|_e| AuthenticationFlowError::PasswordHashingFailure)?;
    let hashed_password = Argon2::default()
        .hash_password(request.password.as_bytes(), &salt_string)
        .map_err(|_e| AuthenticationFlowError::PasswordHashingFailure)?
        .to_string();

    let created_user = state.repository.create_user(&InsertAppUser {
        email: &request.email,
    })?;
    state.repository.create_login_method(&InsertLoginMethod {
        app_user_id: created_user.id,
        login_method_type: LoginMethodType::Password,
        subject_id: None,
        password_hash: Some(&hashed_password),
    })?;
    let (token, expires_at) =
        crate::web::service::auth::generate_jwt(created_user, &state.settings.auth.server_secret)?;
    let response = AuthTokenResponse { token, expires_at };
    Ok((StatusCode::OK, Json(response)))
}

async fn post_auth_signin(
    State(state): State<Arc<ApplicationState>>,
    Json(request): Json<SigninAuthRequest>,
) -> Result<(StatusCode, Json<AuthTokenResponse>), AuthenticationFlowError> {
    let app_user = state
        .repository
        .find_user_by_email(&request.email)?
        .ok_or(AuthenticationFlowError::InvalidCredentials)?;

    // TODO: redirect to password reset flow when is null
    let password_hash = state
        .repository
        .find_login_method(&app_user.id, LoginMethodType::Password)?
        .ok_or(AuthenticationFlowError::NoSuchLoginMethodForUser)?
        .password_hash
        .ok_or(AuthenticationFlowError::IllegalStateException)?;

    let parsed_hash = PasswordHash::new(&password_hash)
        .map_err(|_| AuthenticationFlowError::IllegalStateException)?;

    Argon2::default()
        .verify_password(request.password.as_bytes(), &parsed_hash)
        .map_err(|_| AuthenticationFlowError::InvalidCredentials)?;

    let (token, expires_at) =
        crate::web::service::auth::generate_jwt(app_user, &state.settings.auth.server_secret)?;
    let response = AuthTokenResponse { token, expires_at };
    Ok((StatusCode::OK, Json(response)))
}

async fn post_auth_google(
    State(state): State<Arc<ApplicationState>>,
    Json(request): Json<GoogleAuthRequest>,
) -> Result<(StatusCode, Json<AuthTokenResponse>), AuthenticationFlowError> {
    // GUIDELINES
    // https://developers.google.com/identity/openid-connect/openid-connect?hl=ru

    let mut redis_connection = state.redis.get_connection()?;
    if !redis_connection.exists(&format!("{}:{}", STATE_TOKEN_REDIS_PREFIX, request.state))? {
        return Err(AuthenticationFlowError::InvalidStateToken);
    }

    let params: [(&str, &str); 5] = [
        ("code", &request.code),
        ("client_id", &state.settings.auth.google.client_id),
        ("client_secret", &state.settings.auth.google.client_secret),
        ("redirect_uri", &request.redirect_uri),
        ("grant_type", "authorization_code"),
    ];

    let auth_response = state
        .http_client
        .post(&state.settings.auth.google.token_endpoint)
        .form(&params)
        .send()
        .await?;

    if !auth_response.status().is_success() {
        let auth_response_body = auth_response.text().await?;
        return Err(AuthenticationFlowError::IdentityServerBadResponseStatus {
            body: auth_response_body,
        });
    }

    let auth_response_body = auth_response.json::<serde_json::Value>().await?;

    let token = auth_response_body
        .get("id_token")
        .ok_or(
            AuthenticationFlowError::UnexpectedIdentityServerResponseSchema {
                reason: "Id token was not sent back from google oauth.",
            },
        )?
        .as_str()
        .ok_or(
            AuthenticationFlowError::UnexpectedIdentityServerResponseSchema {
                reason: "Expected id_token to be a string, but it is something else.",
            },
        )?;

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct TokenClaims {
        pub email: String,
        pub aud: String,
        pub iss: String,
        pub exp: u64,
        pub sub: String,
    }

    let token_claims = state.google_jwt_parser.parse::<TokenClaims>(token).await?;

    let app_user: AppUser = match state
        .repository
        .find_user_by_login_method(LoginMethodType::GoogleOauth, &token_claims.sub)?
    {
        Some(u) => Ok::<AppUser, RepositoryError>(u),
        None => {
            let existing = state.repository.find_user_by_email(&token_claims.email)?;
            if let Some(app_user) = existing {
                warn!("User that had other login method signed up through OpenID. Accounts merged.");
                Ok(app_user)
            } else {
                let created_app_user = state.repository.create_user(&InsertAppUser {
                    email: &token_claims.email,
                })?;
                state.repository.create_login_method(&InsertLoginMethod {
                    app_user_id: created_app_user.id,
                    login_method_type: LoginMethodType::GoogleOauth,
                    subject_id: Some(&token_claims.sub),
                    password_hash: None,
                })?;
                Ok(created_app_user)
            }
        }
    }?;
    let (token, expires_at) =
        crate::web::service::auth::generate_jwt(app_user, &state.settings.auth.server_secret)?;
    let response = AuthTokenResponse { token, expires_at };
    Ok((StatusCode::OK, Json(response)))
}
