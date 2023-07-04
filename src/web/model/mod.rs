use serde::Serialize;

pub mod auth;
pub mod graphql;


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonErrorResponse {
    message: String,
    developer_message: String
}
