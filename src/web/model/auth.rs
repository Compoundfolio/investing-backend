use axum::Json;
use axum::extract::rejection::TypedHeaderRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jsonwebtoken_google::ParserError;
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator::ValidationError;

use crate::datasource::diesel::repository::RepositoryError;
use crate::web::model::CommonErrorResponse;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateTokenResponse {
    pub state: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleAuthRequest {
    pub state: String,
    pub code: String,
    pub redirect_uri: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignupAuthRequest {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "validate_password")]
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SigninAuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthTokenResponse {
    pub token: String,
    pub expires_at: usize,
}


#[derive(thiserror::Error, Debug)]
pub enum AuthenticationError {
    #[error("Resource you tried to access requires authentication. Try ot sign in or sign up.")]
    InvalidAuthHeaders { #[from] source: TypedHeaderRejection },
    #[error("Token you provided failed verification process. Most likely, your session is expired. Try signin in again.")]
    InvalidToken { #[from] source: jsonwebtoken::errors::Error }
}

impl IntoResponse for AuthenticationError {
    fn into_response(self) -> Response {
        tracing::error!("Authentication error: {:?}", self);
        use AuthenticationError::*;
        let code = match self {
            InvalidAuthHeaders { .. } => StatusCode::UNAUTHORIZED,
            InvalidToken { .. } => StatusCode::UNAUTHORIZED
        };
        (code, Json(CommonErrorResponse {
            message: self.to_string(),
            details: None,
            developer_message: format!("{self:?}"),
        })).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthenticationFlowError {
    #[error("There was a problem sending requests to the identity servier.")]
    IdentityServerRequestFailure { #[from] source: reqwest::Error, },
    #[error("Identity server responded with an error.")]
    IdentityServerBadResponseStatus { body: String },
    #[error("Identity server sent a response that can't be parsed.")]
    MalformedIdentityServerResponse { #[from] source: ParserError, },
    #[error("Identity server sent a response that doesn't contain required data.")]
    UnexpectedIdentityServerResponseSchema { reason: &'static str },
    #[error("Server failed to generate your access token.")]
    JsonWebTokenFailure { #[from] source: jsonwebtoken::errors::Error, },
    #[error("Server is currently experiencing database connectivity issues.")]
    DatasourceAccessFailure { #[from] source: RepositoryError, },
    #[error("There was a problem connecting to Redis datasource, authentication can't be completed.")]
    RedisConnectionIssue { #[from] source: redis::RedisError, },
    #[error("Request validation failed. Please check information you supplied for the request.")]
    RequestValidationFailed { #[from] source: validator::ValidationErrors, },
    #[error("Server has a problem hashing this password to store it in our database.")]
    PasswordHashingFailure,
    #[error("Server is reading unexpected data from its database.")]
    IllegalStateException,
    #[error("This user already exists, but didn't authorize this login method to be used for authentication.")]
    NoSuchLoginMethodForUser,
    #[error("State token you provided is either expired or invalid. ")]
    InvalidStateToken,
    #[error("This user already exists and can't be registered with a new password. Consider signing in.")]
    UserAlreadyExists,
    #[error("The email and password combination not found.")]
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
        let details = match self {
            InvalidCredentials =>
                Some("Please check your credentials and try once again."),
            InvalidStateToken => 
                Some("This usually happens when you interrupt an authentication flow. \
                Please, try authenticating again."),
            IllegalStateException => 
                Some("This happened because you used an other flow to sign up. \
                Please, add this flow to your account when you are authorized."),
            _ => None
        };
        let details = if code == StatusCode::INTERNAL_SERVER_ERROR && details.is_none() {
            Some(format!("Please, try again later, and if the problem persists, contact our technical support."))
        } else {
            None
        };
        (code, Json(CommonErrorResponse {
            message: self.to_string(),
            details,
            developer_message: format!("{self:?}"),
        })).into_response()
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
