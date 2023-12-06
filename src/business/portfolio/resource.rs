use std::sync::Arc;

use async_graphql::{Context, SimpleObject, InputObject, Object};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::ApplicationState;
use crate::web::routes::graphql::get_claims;

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

impl From<super::model::Portfolio> for Portfolio {
    fn from(value: super::model::Portfolio) -> Self {
        Portfolio { id: value.id, label: value.label }
    }
}


#[derive(Default)]
pub struct PortfolioQuery;
#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl PortfolioQuery {
    /// List of portfolios that belong to you
    async fn portfolios<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<Vec<Portfolio>> {
        let claims = get_claims(ctx)?;
        let state = ctx.data::<Arc<ApplicationState>>()?;
        let portfolios = state.repository.list_portfolios(claims.sub)?;
        Ok(portfolios.into_iter().map(Portfolio::from).collect())
    }
}


#[derive(Default)]
pub struct PortfolioMutation;
#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl PortfolioMutation {
    /// Create a new portfolio
    async fn create_portfolio(&self, ctx: &Context<'_>, data: CreatePortfolio) -> async_graphql::Result<Portfolio> {
        let claims = get_claims(ctx)?;
        let state = ctx.data::<Arc<ApplicationState>>()?;
        let created = state.repository.create_portfolio(claims.sub, &data.label)?;
        Ok(created.into())
    }

    /// Delete portfolio
    async fn delete_portfolio(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Uuid> {
        let claims = get_claims(ctx)?;
        let state = ctx.data::<Arc<ApplicationState>>()?;
        state.repository.delete_portfolio(claims.sub, id)?;
        Ok(id)
    }
}
