use crate::{business::report::model::ReportProcessingError, database::RepositoryError};

use serde::Serialize;

// for REST api
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonErrorResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    pub developer_message: String,
}

#[derive(thiserror::Error, Debug)]
pub enum DescriptiveError {
    #[error("No valid authorization credentials were provided in this request")]
    Unauthorized,
    #[error("Authenticated user does not have access to the desired resource: {0}")]
    Forbidden(String),
    #[error("Resource \"{resource}\" with requested parameters could not be found.")]
    NotFound { resource: String },
    #[error(transparent)]
    RepositoryError( #[from] RepositoryError ),
    #[error(transparent)]
    ReportProcessingError( #[from] ReportProcessingError)
}

impl async_graphql::ErrorExtensions for DescriptiveError {
    // lets define our base extensions
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}", self)).extend_with(|_, e| 
            match self {
                DescriptiveError::Unauthorized => { 
                    e.set("code", "UNAUTHORIZED");
                },
                DescriptiveError::Forbidden(reason) => { 
                    e.set("code", "FORBIDDEN");
                    e.set("reason", reason);
                },
                DescriptiveError::NotFound{..} => {
                    e.set("code", "NOT_FOUND");
                },
                DescriptiveError::RepositoryError(_) => {
                    e.set("code", "REPOSITORY_ERROR");
                },
                DescriptiveError::ReportProcessingError(_) => {
                    e.set("code", "REPORT_PROCESSING_ERROR");
                },
            })
    }
}

