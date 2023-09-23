use serde::Serialize;

pub mod graphql;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonErrorResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    pub developer_message: String,
}
