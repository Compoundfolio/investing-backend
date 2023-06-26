use std::sync::Arc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Extension, Router};
use axum_macros::debug_handler;

use crate::web::model::graphql::ServiceSchema;
use crate::ApplicationState;
use crate::web::service;

pub fn routes() -> Router<Arc<ApplicationState>> {
    Router::new().route("/", get(graphql_playground).post(graphql_handler))
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

#[debug_handler]
async fn graphql_handler(
    Extension(schema): Extension<ServiceSchema>,
    claims: service::auth::AuthClaims,
    State(_): State<Arc<ApplicationState>>,
    req: GraphQLRequest
) -> GraphQLResponse {
    schema.execute(req.into_inner().data(claims)).await.into()
}
