use axum::extract::rejection::TypedHeaderRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jsonwebtoken_google::ParserError;
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator::ValidationError;

use crate::datasource::diesel::repository::RepositoryError;

#[derive(Debug, Serialize)]
pub struct StateTokenResponse {
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleAuthRequest {
    pub state: String,
    pub code: String,
    pub redirect_uri: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SignupAuthRequest {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "validate_password")]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SigninAuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthTokenResponse {
    pub token: String,
    pub expires_at: usize,
}


#[derive(thiserror::Error, Debug)]
pub enum AuthenticationError {
    #[error("This endpoint requires you to include the following headers in your request: {source:?}")]
    InvalidAuthHeaders { #[from] source: TypedHeaderRejection },
    #[error("Token you provided failed verification: {source}")]
    InvalidToken { #[from] source: jsonwebtoken::errors::Error }
}

impl IntoResponse for AuthenticationError {
    fn into_response(self) -> Response {
        // TODO: Add logging, as auth errors may be sign of broken system or malicouse attocks
        use AuthenticationError::*;
        let code = match self {
            InvalidAuthHeaders { .. } => StatusCode::UNAUTHORIZED,
            InvalidToken { .. } => StatusCode::UNAUTHORIZED
        };
        (code, self.to_string()).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthenticationFlowError {
    #[error("There was a problem sending requests to the identity servier: {source} {source:?}")]
    IdentityServerRequestFailure { #[from] source: reqwest::Error, },
    #[error("Identity server responded with an error: {body:?}")]
    IdentityServerBadResponseStatus { body: String },
    #[error("Identity server sent a response that can't be parsed: {source:?}")]
    MalformedIdentityServerResponse { #[from] source: ParserError, },
    #[error("Identity server sent a response that doesn't contain required data: {reason}")]
    UnexpectedIdentityServerResponseSchema { reason: &'static str },
    #[error("Server failed to generate a JWT: {source:?}")]
    JsonWebTokenFailure { #[from] source: jsonwebtoken::errors::Error, },
    #[error("Server failed at accessing its datadource: {source}")]
    DatasourceAccessFailure { #[from] source: RepositoryError, },
    #[error("There was a problem connecting to Redis datasource, authentication can't be completed: {source}")]
    RedisConnectionIssue { #[from] source: redis::RedisError, },
    #[error("Request is invalid:\n{source}")]
    RequestValidationFailed { #[from] source: validator::ValidationErrors, },
    #[error("Server has a problem hashing this password to store it in its database.")]
    PasswordHashingFailure,
    #[error("Server is readin unexpected data from its database. Please, try again later, and if the problem\
             persists, contact our technical support.")]
    IllegalStateException,
    #[error("This user already exists, but didn't authorize this login method to be used for authentication.\
             Please, add this login method to your account when you are authorized.")]
    NoSuchLoginMethodForUser,
    #[error("State token you provided is either expired or invalid. Request a new state token from server before\
             starting the OpenID authentication flow.")]
    InvalidStateToken,
    #[error("This user already exists and can't be registered with a new password. Consider signing in.")]
    UserAlreadyExists,
    #[error("The email and password combination you entered didn't match our records. Please check your credentials and try again.")]
    InvalidCredentials,
    
}

impl IntoResponse for AuthenticationFlowError {
    fn into_response(self) -> Response {
        tracing::error!("Authentication flow error: {:?}", self);
        use AuthenticationFlowError::*;
        let code = match self {
            RequestValidationFailed { .. } => StatusCode::BAD_REQUEST,
            NoSuchLoginMethodForUser => StatusCode::FORBIDDEN,
            InvalidStateToken => StatusCode::FORBIDDEN,
            UserAlreadyExists => StatusCode::CONFLICT,
            InvalidCredentials => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (code, self.to_string()).into_response()
    }
}

fn validate_password(pass: &str) -> Result<(), ValidationError> {
    use passwords::analyzer;
    let analyzed = analyzer::analyze(pass);
    if analyzed.length() < 8 {
        return Err(ValidationError::new(
            "Password should be at least 8 characters long",
        ));
    }
    if analyzed.lowercase_letters_count() < 1 {
        return Err(ValidationError::new(
            "Password should contain at least 1 lowercase letter",
        ));
    }
    if analyzed.uppercase_letters_count() < 1 {
        return Err(ValidationError::new(
            "Password should contain at least 1 uppercase letter",
        ));
    }
    if analyzed.symbols_count() < 1 {
        return Err(ValidationError::new(
            "Password should contain at least 1 symbol",
        ));
    }
    Ok(())
}
