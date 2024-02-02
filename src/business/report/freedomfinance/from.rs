use serde_json::json;

use crate::business::report::model::BrokerType;

use super::super::model::{AbstractReport, AbstractTradeOperation, AbstractTransaction, AbstractTradeSide, AbstractTransactionType, AbstractOperationSource, Money};
use super::model::CashInOutType;

impl From<super::model::Report> for AbstractReport {
    fn from(value: super::model::Report) -> Self {
        Self {
            trade_operations: value.trades.detailed.into_iter().map(|v| v.into()).collect(),
            transactions: value.cash_in_outs.into_iter().map(|v| v.into()).collect(),
            broker: BrokerType::Freedomfinance
        }
    }
}

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
        let commission = if let Some(currency) = value.commission_currency {
            if bigdecimal::Zero::is_zero(&value.commission) {
                None
            } else {
                Some(Money::new(value.commission, currency))
            }
        } else {
            None
        };

        Self {
            operation_source: AbstractOperationSource::FreedomfinanceReport,
            external_id: Some(value.id.to_string()),
            date_time: value.datetime,
            symbol_id: value.ticker,
            amount: Money::new(value.amount, value.currency),
            operation_type: match value.operation_type {
                CashInOutType::DividendReverted => AbstractTransactionType::RevertedDividend,
                CashInOutType::Dividend => AbstractTransactionType::Dividend,
                CashInOutType::Card => AbstractTransactionType::FundingWithdrawal,
                CashInOutType::Unrecognized(a) => AbstractTransactionType::Unrecognized(a),
            },
            commission,
            metadata: json!({
                "transaction_id": value.transaction_id.to_string(),
                "details": value.details,
                "value_usd_details": value.value_usd_details,
                "reverted": value.reverted.to_string(),
            }),
        }
    }
}
