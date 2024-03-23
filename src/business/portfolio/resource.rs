use async_graphql::{Context, InputObject, Object};
use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

use crate::{business::model::{BrokerType, Money}, web::{errors::DescriptiveError, graphql::{get_claims, get_state}}};

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
    async fn amount_of_user_transactions<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<i64> {
        let state = get_state(ctx)?;
        Ok(super::super::user_transaction::service::count_user_transactions(state, self.id)?)
    }
    async fn total_return_percentage(&self) -> async_graphql::Result<Decimal> {
        Ok(Decimal::ZERO)
    }
    async fn total_return_value(&self) -> async_graphql::Result<Money> {
        Ok(Money { amount: Decimal::ZERO, currency: "USD".to_string() })
    }
    async fn annual_income(&self) -> async_graphql::Result<Money> {
        Ok(Money { amount: Decimal::ZERO, currency: "USD".to_string() })
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
