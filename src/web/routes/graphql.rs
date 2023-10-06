use std::sync::Arc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, Object, Schema, Upload, UploadValue, InputObject};
use async_graphql::EmptySubscription;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_graphql::SimpleObject;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Extension, Router};

use crate::auth::service::AuthClaims;
use crate::ApplicationState;

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

pub type ServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;


pub struct QueryRoot;
#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl QueryRoot {

    /// Information about you as a signed in user
    async fn me<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<Me<'ctx>> {
        let claims = get_claims(&ctx)?;
        Ok(Me {
            email: claims.email.as_str(),
        })
    }

    /// List of portfolios that belong to you
    async fn portfolios<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<Vec<Portfolio>> {
        let claims = get_claims(&ctx)?;
        let state = ctx.data::<Arc<ApplicationState>>()?;
        let portfolios = state.repository.list_portfolios(claims.sub)?;
        Ok(portfolios.into_iter().map(Portfolio::from).collect())
    }
}

pub fn get_claims<'ctx>(ctx: &Context<'ctx>) -> async_graphql::Result<&'ctx AuthClaims> {
    return Ok(ctx
        .data::<Claims>()?
        .as_ref()
        .ok_or_else(|| async_graphql::Error::new("You are not authorized."))?);
}

pub struct MutationRoot;
#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl MutationRoot {

    /// Create a new portfolio
    async fn create_portfolio(&self, ctx: &Context<'_>, data: CreatePortfolio) -> async_graphql::Result<Portfolio> {
        let claims = get_claims(&ctx)?;
        let state = ctx.data::<Arc<ApplicationState>>()?;
        let created = state.repository.create_portfolio(claims.sub, &data.label)?;
        Ok(created.into())
    }

    /// Delete portfolio
    async fn delete_portfolio(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Uuid> {
        let claims = get_claims(&ctx)?;
        let state = ctx.data::<Arc<ApplicationState>>()?;
        state.repository.delete_portfolio(claims.sub, id)?;
        Ok(id)
    }

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


pub fn do_something_with_async<R: futures_util::io::AsyncRead + Unpin>(_reader: R) {}

#[derive(Serialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Me<'a> {
    pub email: &'a str,
}

#[derive(Serialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Portfolio {
    pub id: Uuid,
    pub label: String,
}

#[derive(Deserialize, InputObject)]
#[serde(rename_all = "camelCase")]
pub struct CreatePortfolio {
    pub label: String
}

impl From<crate::portfolio::model::Portfolio> for Portfolio {
    fn from(value: crate::portfolio::model::Portfolio) -> Self {
        Portfolio { id: value.id, label: value.label }
    }
}
