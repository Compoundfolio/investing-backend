use std::sync::Arc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, Object, Schema, Upload, UploadValue};
use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Extension, Router};

use crate::web::model::graphql::Me;
use crate::web::service;
use crate::web::service::auth::AuthClaims;
use crate::ApplicationState;

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
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner().data(claims)).await.into()
}

pub type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;
#[Object]
impl QueryRoot {
    async fn me<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<Me<'ctx>> {
        let claims: &AuthClaims = ctx
            .data::<Claims>()?
            .as_ref()
            .ok_or_else(|| async_graphql::Error::new("You are not authorized."))?;
        Ok(Me {
            email: claims.email.as_str(),
        })
    }
}

struct MutationRoot;
#[Object]
impl MutationRoot {
    async fn upload_file(
        &self,
        ctx: &Context<'_>,
        upload: Upload,
    ) -> async_graphql::Result<String> {
        let upload_value: UploadValue = upload.value(ctx)?;
        let async_read = upload_value.into_async_read();
        do_something_with_async(async_read);

        Ok("OK".to_owned())
    }
}

fn do_something_with_async<R: futures_util::io::AsyncRead + Unpin>(_reader: R) {}
