use serde_json::json;

use crate::portfolio::model::{AbstractTradeOperation, AbstractTransaction, AbstractTradeSide, AbstractTransactionType, AbstractOperationSource, Money};

use super::model::CashInOutType;


impl From<super::model::DetailedTrade> for AbstractTradeOperation {
    fn from(value: super::model::DetailedTrade) -> Self {
        Self { 
            operation_source: AbstractOperationSource::FreedomfinanceReport,
            external_id: Some(value.id.to_string()),
            date_time: value.date,
            side: match value.operation { 
                super::model::TradeOperationSide::Buy => AbstractTradeSide::Buy,
                super::model::TradeOperationSide::Sell => AbstractTradeSide::Sell,
            },
            instrument_symbol: value.instr_nm,
            isin: value.isin,
            price: Money::new(value.price, value.curr_c.clone()),
            quantity: value.quantity,
            commission: Money::new(value.commission, value.commission_currency),
            order_id: value.order_id.to_string(),
            summ: Money::new(value.summ, value.curr_c),
            metadata: json!({
                "comment": value.comment,
                "market": value.mkt_name,
                "instr_kind": value.instr_kind,
                "trade_id": value.trade_id.to_string(),
            })
        }
    }
}

impl From<super::model::CashInOut> for AbstractTransaction {
    fn from(value: super::model::CashInOut) -> Self {
        Self {
            source: AbstractOperationSource::FreedomfinanceReport,
            external_id: Some(value.id.to_string()),
            date_time: value.datetime,
            symbol_id: value.ticker,
            amount: value.amount,
            currency: value.currency,
            operation_type: match value.operation_type {
                CashInOutType::DividentReverted => AbstractTransactionType::RevertedDivident,
                CashInOutType::Divident => AbstractTransactionType::Divident,
                CashInOutType::Card => AbstractTransactionType::FundingWithdrawal,
                CashInOutType::Unrecognized(a) => AbstractTransactionType::Unrecognized(a),
            },
            comission: match value.commission {
                some if some.to_string() == "0"  => None,
                some => Some(some)
            },
            comission_currency: value.commission_currency,
            metadata: json!({
                "transaction_id": value.transaction_id.to_string(),
                "details": value.details,
                "value_usd_details": value.value_usd_details,
                "reverted": value.reverted.to_string(),
            }),
        }
    }
}
