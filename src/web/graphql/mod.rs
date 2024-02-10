
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
use crate::business::user_transaction::resource::{UserTransactionMutation, UserTransactionQuery};
use crate::ApplicationState;
use crate::business::portfolio::resource::{PortfolioQuery, PortfolioMutation};
use crate::business::report::resource::ReportMutation;

pub mod errors;

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
        .ok_or_else(|| errors::DescriptiveError::Unauthorized.extend() );
}

pub fn get_state<'ctx>(ctx: &Context<'ctx>) -> async_graphql::Result<&'ctx Arc<ApplicationState>> {
    return ctx.data::<Arc<ApplicationState>>();
}


// --- default and miscellaneous queries and mutations

#[derive(MergedObject, Default)]
pub struct QueryRoot(MiscellaneousQuery, PortfolioQuery, UserTransactionQuery);
#[derive(MergedObject, Default)]
pub struct MutationRoot( /* MiscellaneousMutation, */ PortfolioMutation, ReportMutation, UserTransactionMutation);
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

