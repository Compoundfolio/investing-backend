use std::sync::Arc;

use async_graphql::{Context, Object, Schema};
use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Extension, Router};

use crate::ApplicationState;
use crate::web::model::graphql::Me;
use crate::web::service;
use crate::web::service::auth::AuthClaims;

pub fn routes() -> Router<Arc<ApplicationState>> {
    Router::new().route("/graphql", get(graphql_playground).post(graphql_handler))
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql/ws"),
    ))
}

type Claims = Option<service::auth::AuthClaims>;
async fn graphql_handler(
    Extension(schema): Extension<ServiceSchema>,
    claims: Claims,
    State(_): State<Arc<ApplicationState>>,
    req: GraphQLRequest
) -> GraphQLResponse {
    schema.execute(req.into_inner().data(claims)).await.into()
}

pub type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
pub struct QueryRoot;


#[Object]
impl QueryRoot {
    async fn me(&self, ctx: &Context<'_>) -> async_graphql::Result<Me> {
        let claims: &AuthClaims= ctx.data::<Claims>()?
            .as_ref()
            .ok_or_else(|| async_graphql::Error::new("You are not authorized."))?;

        Ok(Me {
            email: claims.email.clone()
        })
    }
}
