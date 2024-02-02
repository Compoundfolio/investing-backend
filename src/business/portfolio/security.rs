use uuid::Uuid;

use crate::{ApplicationState, web::graphql::DescriptiveError};


pub fn is_portfolio_owner(state: &ApplicationState, app_user_id: Uuid, portfolio_id: Uuid) -> Result<(), DescriptiveError> {
    let portfolio = state.repository.find_portfolio_by_id(portfolio_id)?
        .ok_or(DescriptiveError::NotFound { resource: "portfolio".to_string() })?;
    if portfolio.app_user_id != app_user_id {
        Err(DescriptiveError::Forbidden("This portfolio belongs to an other user".to_string()))
    } else {
        Ok(())
    }

}
