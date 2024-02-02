use std::sync::Arc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, Object, Schema, MergedObject, ErrorExtensions};
use async_graphql::EmptySubscription;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_graphql::SimpleObject;
use serde::Serialize;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Extension, Router};

use crate::auth::service::AuthClaims;
use crate::ApplicationState;
use crate::business::portfolio::resource::{PortfolioQuery, PortfolioMutation};
use crate::business::report::model::ReportProcessingError;
use crate::business::report::resource::ReportMutation;
use crate::database::RepositoryError;

// --- configurations of GraphQL

pub fn routes() -> Router<Arc<ApplicationState>> {
    Router::new().route("/graphql", get(graphql_playground).post(graphql_handler))
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql/ws"),
    ))
}

type Claims = Option<AuthClaims>;
async fn graphql_handler(
    Extension(schema): Extension<ServiceSchema>,
    claims: Claims,
    State(state): State<Arc<ApplicationState>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner().data(claims).data(state)).await.into()
}

// --- convinience functions and utils

pub fn get_claims<'ctx>(ctx: &Context<'ctx>) -> async_graphql::Result<&'ctx AuthClaims> {
    return ctx
        .data::<Claims>()?
        .as_ref()
        .ok_or_else(|| DescriptiveError::Unauthorized.extend() );
}

pub fn get_state<'ctx>(ctx: &Context<'ctx>) -> async_graphql::Result<&'ctx Arc<ApplicationState>> {
    return ctx.data::<Arc<ApplicationState>>();
}

// --- common error structs

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
                DescriptiveError::RepositoryError(RepositoryError::NoRowsAffected) => {
                    e.set("code", "NO_ROWS_AFFECTED");
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

// --- default and miscellaneous queries and mutations

#[derive(MergedObject, Default)]
pub struct QueryRoot(MiscellaneousQuery, PortfolioQuery);
#[derive(MergedObject, Default)]
pub struct MutationRoot( /* MiscellaneousMutation, */ PortfolioMutation, ReportMutation);
pub type ServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;



#[derive(Default)]
struct MiscellaneousQuery;
#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl MiscellaneousQuery {
    /// Information about you as a signed in user
    async fn me<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<Me<'ctx>> {
        let claims = get_claims(ctx)?;
        Ok(Me {
            email: claims.email.as_str(),
        })
    }

}

/*
#[derive(Default)]
struct MiscellaneousMutation;
#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl MiscellaneousMutation {
}
*/

#[derive(Serialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Me<'a> {
    pub email: &'a str,
}

