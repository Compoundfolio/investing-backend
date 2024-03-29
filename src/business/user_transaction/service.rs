use uuid::Uuid;

use crate::{business::{user_transaction::resource::UserTransaction}, web::errors::DescriptiveError, ApplicationState};


pub fn generate_user_transaction_list(state: &ApplicationState, portfolio_id: Uuid) -> Result<Vec<UserTransaction>, DescriptiveError> {
    let trade_operations = state.repository.list_trade_operations(portfolio_id)?;
    let fiscal_transactions = state.repository.list_fiscal_transactions(portfolio_id)?;

    let mut all_user_transactions: Vec<UserTransaction> = trade_operations.into_iter()
            .map(UserTransaction::from)
            .chain(fiscal_transactions
                    .into_iter()
                    .map(UserTransaction::from)
            )
            .collect();

    all_user_transactions.sort_unstable_by_key(|uo| uo.date_time);
    Ok(all_user_transactions)
}

pub fn count_user_transactions(state: &ApplicationState, portfolio_id: Uuid) -> Result<i64, DescriptiveError> {
    let trade_operations = state.repository.count_trade_operations(portfolio_id)?;
    let fiscal_transactions = state.repository.count_trade_operations(portfolio_id)?;
    return Ok(trade_operations + fiscal_transactions);

}
