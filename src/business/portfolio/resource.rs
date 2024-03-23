use async_graphql::{Context, InputObject, Object};
use serde::Deserialize;
use uuid::Uuid;

use crate::{business::model::BrokerType, web::{errors::DescriptiveError, graphql::{get_claims, get_state}}};

pub struct Portfolio {
    pub id: Uuid,
    pub title: String
}

#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl Portfolio {
    async fn id(&self) -> Uuid { self.id }
    async fn title(&self) -> String { self.title.clone() }
    async fn brokerages<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<Vec<BrokerType>> {
        let state = get_state(ctx)?;
        Ok(state.repository.list_portfolio_brokerages(self.id)?)
    }
}

#[derive(Deserialize, InputObject)]
#[serde(rename_all = "camelCase")]
pub struct CreatePortfolio {
    pub title: String
}

impl From<super::model::SelectPortfolio> for Portfolio {
    fn from(value: super::model::SelectPortfolio) -> Self {
        Portfolio { id: value.id, title: value.label }
    }
}


#[derive(Default)]
pub struct PortfolioQuery;
#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl PortfolioQuery {
    /// List of portfolios that belong to you
    async fn portfolios<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<Vec<Portfolio>> {
        let claims = get_claims(ctx)?;
        let state = get_state(ctx)?;
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
        let state = get_state(ctx)?;
        let created = state.repository.create_portfolio(claims.sub, &data.title)?;
        Ok(created.into())
    }

    /// Delete portfolio
    async fn delete_portfolio(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Uuid> {
        let claims = get_claims(ctx)?;
        let state = get_state(ctx)?;
        let affected = state.repository.delete_portfolio(claims.sub,id)? ;
        match affected {
            0 => Err(DescriptiveError::NotFound { resource: "portfolio".to_owned() }.into()),
            _ => Ok(id)
        }
    }
}
