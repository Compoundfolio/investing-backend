use async_graphql::{Context, CustomValidator, InputObject, InputValueError, Object};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::business::portfolio::security::is_portfolio_owner;
use crate::business::model::{BrokerType, Money};
use crate::web::graphql::errors::DescriptiveError;
use crate::web::graphql::{get_claims, get_state};

use super::model::{FiscalTransaction, FiscalTransactionType, InsertFiscalTransaction};


#[derive(Default)]
pub struct FiscalTransactionMutation;
#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl FiscalTransactionMutation {
    /// Create a new fiscal transaction within a portfolio identified by id
    async fn create_fiscal_transaction(
        &self, 
        ctx: &Context<'_>, 
        #[graphql(validator(custom = "CreateFiscalTransactionValidator{}"))]
        create_request: CreateFiscalTransaction 
    ) -> async_graphql::Result<Uuid> {
        let claims = get_claims(ctx)?;
        let state = get_state(ctx)?;

        is_portfolio_owner(state, claims.sub, create_request.portfolio_id)?;
        let created = state.repository.create_fiscal_transaction(create_request.into())?;
        Ok(created)
    }

    /// Delete a fiscal transaction transaction
    async fn delete_fiscal_transaction(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<String> {
        let claims = get_claims(ctx)?;
        let state = get_state(ctx)?;
        let to_be_deleted = state.repository.find_fiscal_transaction_by_id(id)?
            .ok_or(DescriptiveError::NotFound { resource: "fiscal transaction".to_owned() })?;
        is_portfolio_owner(state, claims.sub, to_be_deleted.portfolio_id)?;
        state.repository.delete_fiscal_transaction(id)?;
        Ok("OK".to_owned())
    }
}

// --- model


#[derive(InputObject,Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateFiscalTransaction {
    pub portfolio_id: Uuid,
    /// Associate this transaction with a broker. 
    pub brokerage: Option<BrokerType>,
    /// Date and time at which this transaction occurred in a local timezone
    pub date_time: NaiveDateTime,
    /// Associate this transaction with an instrument. Field value is validated.
    /// Must be present for DIVIDEND. Must be null for FUNDIG_WITHDRAWAL.
    pub instrument_symbol: Option<String>,
    /// Total transaction amount, negative if money is being withdrawn.
    /// Must be negative for TAX and COMISSION.
    /// Must be positive for DIVIDEND.
    pub amount: Money,
    pub transaction_type: CreateableFiscalTransactionType,
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
enum CreateableFiscalTransactionType {
    Tax, Dividend, FundingWithdrawal, Comission
}

impl Into<InsertFiscalTransaction> for CreateFiscalTransaction {
    fn into(self) -> InsertFiscalTransaction {
        InsertFiscalTransaction {
            portfolio_id: self.portfolio_id,
            report_upload_id: None,
            fiscal_transaction: FiscalTransaction {
                operation_source: crate::business::model::OperationSource::Manual,
                broker: self.brokerage,
                external_id: None,
                date_time: self.date_time,
                symbol_id: self.instrument_symbol,
                amount: self.amount,
                operation_type: self.transaction_type.into(),
                commission: None,
                metadata: serde_json::Value::Null,
            },
        }
    }
}

impl Into<FiscalTransactionType> for CreateableFiscalTransactionType {
    fn into(self) -> FiscalTransactionType {
        match self {
            CreateableFiscalTransactionType::Tax => FiscalTransactionType::Tax,
            CreateableFiscalTransactionType::Dividend => FiscalTransactionType::Dividend,
            CreateableFiscalTransactionType::FundingWithdrawal => FiscalTransactionType::FundingWithdrawal,
            CreateableFiscalTransactionType::Comission => FiscalTransactionType::Commission,
        }
    }
}

// --- validation

struct CreateFiscalTransactionValidator { }

impl CustomValidator<CreateFiscalTransaction> for CreateFiscalTransactionValidator {
    fn check(&self, value: &CreateFiscalTransaction) -> Result<(), InputValueError<CreateFiscalTransaction>> {
        match value.transaction_type {
            CreateableFiscalTransactionType::Dividend => {
                if value.instrument_symbol.is_none() {
                    Err(InputValueError::custom("DIVIDEND transaction must be associated with an instrument"))
                } else if value.amount.amount.is_sign_negative() {
                    Err(InputValueError::custom("DIVIDEND transaction must contain positive amount"))
                } else {
                    Ok(())
                }
            },
            CreateableFiscalTransactionType::FundingWithdrawal => {
                if value.instrument_symbol.is_some() {
                    Err(InputValueError::custom("FUNDING_WITHDRAWAL transaction can't be associated with an instrument"))
                } else {
                    Ok(())
                }
            },
            CreateableFiscalTransactionType::Tax => {
                if value.amount.amount.is_sign_positive() {
                    Err(InputValueError::custom("TAX transaction must contain negative amount"))
                } else {
                    Ok(())
                }
            },
            CreateableFiscalTransactionType::Comission => {
                if value.amount.amount.is_sign_positive() {
                    Err(InputValueError::custom("COMISSION transaction must contain negative amount"))
                } else {
                    Ok(())
                }
            }
        }
    }
}
