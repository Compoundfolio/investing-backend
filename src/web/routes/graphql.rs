use std::sync::Arc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, Object, Schema, Upload, UploadValue, MergedObject};
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
use crate::portfolio::resource::{PortfolioQuery, PortfolioMutation};

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

pub fn get_claims<'ctx>(ctx: &Context<'ctx>) -> async_graphql::Result<&'ctx AuthClaims> {
    return Ok(ctx
        .data::<Claims>()?
        .as_ref()
        .ok_or_else(|| async_graphql::Error::new("You are not authorized."))?);
}


#[derive(MergedObject, Default)]
pub struct QueryRoot(MiscellaneousQuery, PortfolioQuery);
#[derive(MergedObject, Default)]
pub struct MutationRoot(MiscellaneousMutation, PortfolioMutation);
pub type ServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;



#[derive(Default)]
struct MiscellaneousQuery;
#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl MiscellaneousQuery {
    /// Information about you as a signed in user
    async fn me<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<Me<'ctx>> {
        let claims = get_claims(&ctx)?;
        Ok(Me {
            email: claims.email.as_str(),
        })
    }

}


#[derive(Default)]
struct MiscellaneousMutation;
#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl MiscellaneousMutation {
    #[graphql(deprecation = true)]
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

#[derive(Serialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Me<'a> {
    pub email: &'a str,
}

