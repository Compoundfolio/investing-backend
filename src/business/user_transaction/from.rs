use crate::business::report::model::{SelectFiscalTransaction, FiscalTransactionType, SelectTradeOperation, TradeOperationSide};

use super::resource::{UserTransaction, UserTransactionTradeSide, UserTransactionType};


impl From<FiscalTransactionType> for UserTransactionType {
    fn from(value: FiscalTransactionType) -> Self {
        match value {
            FiscalTransactionType::Tax => UserTransactionType::Tax,
            FiscalTransactionType::Dividend => UserTransactionType::Divident,
            FiscalTransactionType::Commission => UserTransactionType::Comission,
            FiscalTransactionType::FundingWithdrawal => UserTransactionType::FundingWithdrawal,
            FiscalTransactionType::RevertedDividend => UserTransactionType::RevertedDivident,
            FiscalTransactionType::Unrecognized(_) => UserTransactionType::Unrecognized
        }
    }
}



impl From<SelectFiscalTransaction> for UserTransaction {
    fn from(value: SelectFiscalTransaction) -> Self {
        Self {
            unrecognized_type: if let FiscalTransactionType::Unrecognized(ref t) = value.i.operation_type { Some(t.clone()) } else { None },
            user_transaction_type: value.i.operation_type.into(),
            date_time: value.i.date_time,
            summ: value.i.amount,
            symbol: value.i.symbol_id,
            price: None,
            quantity: None,
            trade_side: None,
            trade_operation_id: None,
            fiscal_transaction_id: Some(value.id),
        }
    }
}

impl From<TradeOperationSide> for UserTransactionTradeSide {
    fn from(value: TradeOperationSide) -> Self {
        match value {
            TradeOperationSide::Buy => UserTransactionTradeSide::Buy,
            TradeOperationSide::Sell => UserTransactionTradeSide::Sell
        }
    }
}

impl From<SelectTradeOperation> for UserTransaction {
    fn from(value: SelectTradeOperation) -> Self {
        let operation_signum = match value.i.side {
            crate::business::report::model::TradeOperationSide::Buy => -1,
            crate::business::report::model::TradeOperationSide::Sell => 1,
        };
        Self {
            user_transaction_type: UserTransactionType::Trade,
            date_time: value.i.date_time,
            summ: value.i.summ * operation_signum,
            symbol: Some(value.i.instrument_symbol),
            price: Some(value.i.price),
            quantity: Some(value.i.quantity),
            trade_side: Some(value.i.side.into()),
            trade_operation_id: Some(value.id),
            fiscal_transaction_id: None,
            unrecognized_type: None,
        }
    }
}
