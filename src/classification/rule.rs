use serde::{Deserialize, Serialize};

use crate::{classification::TransactionKind, transaction::Transaction};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClassificationRule {
    Exact {
        value: String,
        kind: TransactionKind,
    },
    Contains {
        value: String,
        kind: TransactionKind,
    },
}

impl ClassificationRule {
    pub fn matches(&self, transaction: &Transaction) -> bool {
        match self {
            ClassificationRule::Exact { value, .. } => transaction.name.eq_ignore_ascii_case(value),
            ClassificationRule::Contains { value, .. } => transaction
                .name
                .to_lowercase()
                .contains(&value.to_lowercase()),
        }
    }

    pub fn kind(&self) -> TransactionKind {
        match self {
            ClassificationRule::Exact { kind, .. } => *kind,
            ClassificationRule::Contains { kind, .. } => *kind,
        }
    }
}
