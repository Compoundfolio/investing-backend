use serde_json::json;

use crate::business::{fiscal_transaction::model::FiscalTransactionType, model::{BrokerType, Money, OperationSource}, report::model::AbstractReport, trade_operation::model::TradeOperationSide};

use super::model::TransactionOperationType;

use crate::business::{fiscal_transaction::model::FiscalTransaction, trade_operation::model::TradeOperation};

impl From<super::model::Report> for AbstractReport {
    fn from(value: super::model::Report) -> Self {
        Self {
            trade_operations: value.trade_operations.into_iter().map(|v| v.into()).collect(),
            fiscal_transactions: value.transactions.into_iter().map(|v| v.into()).collect(),
            broker: BrokerType::Exante
        }
    }
}

impl From<super::model::TradeOperation> for TradeOperation {
    fn from(value: super::model::TradeOperation) -> Self {
        Self { 
            operation_source: OperationSource::ExanteReport,
            broker: Some(BrokerType::Exante),
            external_id: Some(format!("{}/{}", value.order_id, value.order_pos)),
            date_time: value.timestamp,
            side: match value.side {
                super::model::TradeOperationSide::Buy => TradeOperationSide::Buy,
                super::model::TradeOperationSide::Sell => TradeOperationSide::Sell,
            },
            instrument_symbol: value.symbol_id,
            isin: Some(value.isin),
            price: Money::new(value.price, value.currency.clone()),
            quantity: value.quantity,
            commission: Some(Money::new(value.commission, value.commission_currency)),
            order_id: Some(value.order_id.to_string()),
            summ: Money::new(value.traded_volume, value.currency),
            metadata: json!({
                "uti": value.uti,
                "trade_type": value.trade_type,
                "type": value.trade_operation_type,
                "order_pos": value.order_pos.to_string(),
                "account_id": value.account_id,
            })
        }
    }
}

impl From<super::model::Transaction> for FiscalTransaction {
    fn from(value: super::model::Transaction) -> Self {
        Self {
            operation_source: OperationSource::ExanteReport,
            broker: Some(BrokerType::Exante),
            external_id: Some(value.id),
            date_time: value.timestamp,
            symbol_id: match value.symbol_id.as_str() {
                "None" => None,
                some => Some(some.to_string())
            },
            operation_type: match value.operation_type {
                TransactionOperationType::UsTax => FiscalTransactionType::Tax,
                TransactionOperationType::Tax => FiscalTransactionType::Tax,
                TransactionOperationType::Dividend => FiscalTransactionType::Dividend,
                TransactionOperationType::Trade => FiscalTransactionType::Unrecognized("Trade".to_owned()),
                TransactionOperationType::Commission => FiscalTransactionType::Commission,
                TransactionOperationType::FundingWithdrawal => FiscalTransactionType::FundingWithdrawal,
                TransactionOperationType::Unrecognized(a) => FiscalTransactionType::Unrecognized(a),
            },
            amount: Money::new(value.sum, value.asset),
            commission: None,
            metadata: json!({
                "account_id": value.account_id,
                "isin": value.isin,
                "eur_equivalent": value.eur_equivalent.to_string(),
                "comment": value.comment.to_string(),
                "uuid": value.uuid,
                "parent_uuid": value.parent_uuid
            }),
        }
    }
}
