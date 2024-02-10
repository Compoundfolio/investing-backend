use uuid::Uuid;

use crate::{web::graphql::errors::DescriptiveError, ApplicationState};

// struct PortfolioOwnerGuard { }
// #[async_trait::async_trait]
// impl Guard for RoleGuard {
//     async fn check(&self, ctx: &Context<'_>) -> Result<()> {
//         // you need to exctract portolio ID to implement this
//         let claims = get_claims(ctx)?;
//         let state = get_state(ctx)?;
//         return is_portfolio_owner(state, claims.sub, portfolio_id)
//     }
// }


pub fn is_portfolio_owner(state: &ApplicationState, app_user_id: Uuid, portfolio_id: Uuid) -> Result<(), DescriptiveError> {
    let portfolio = state.repository.find_portfolio_by_id(portfolio_id)?
        .ok_or(DescriptiveError::NotFound { resource: "portfolio".to_string() })?;
    if portfolio.app_user_id != app_user_id {
        Err(DescriptiveError::Forbidden("This portfolio belongs to an other user".to_string()))
    } else {
        Ok(())
    }

}
