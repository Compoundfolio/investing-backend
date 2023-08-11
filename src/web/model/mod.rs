use serde::Serialize;

pub mod auth;
pub mod graphql;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonErrorResponse {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
    developer_message: String,
}
