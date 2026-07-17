use core::fmt;
use std::{error::Error, fmt::Display, str::FromStr};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::classification::TransactionKind;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// The unique identifier of a transaction
    pub id: Uuid,
    pub date: DateTime<Local>,
    pub name: String,
    pub trans_type: TransactionType,
    /// Amount of the transaction in Euros
    ///
    /// The format is stored as an integer to make sure no precision is lost
    /// As an example, 20 euros is stored as 2000
    pub amount: i64,

    pub kind: Option<TransactionKind>,
}

#[derive(Debug)]
pub struct ParseTransactionTypeError {
    value: String,
}

impl Display for ParseTransactionTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown transaction type: {}", self.value)
    }
}

impl Error for ParseTransactionTypeError {}

impl FromStr for TransactionType {
    type Err = ParseTransactionTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "debit" => Ok(TransactionType::Debit),
            "credit" => Ok(TransactionType::Credit),
            _ => Err(ParseTransactionTypeError {
                value: s.to_owned(),
            }),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TransactionType {
    Debit,
    Credit,
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Debit => write!(f, "Debit"),
            TransactionType::Credit => write!(f, "Credit"),
        }
    }
}
