use serde::{Deserialize, Serialize};

use crate::{
    classification::TransactionKind,
    transaction::{Transaction, TransactionType},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClassificationRule {
    Exact {
        value: String,
        kind: TransactionKind,
        trans_type: TransactionType,
    },
    Contains {
        value: String,
        kind: TransactionKind,
        trans_type: TransactionType,
    },
}

impl ClassificationRule {
    pub fn matches(&self, transaction: &Transaction) -> bool {
        match self {
            ClassificationRule::Exact {
                value, trans_type, ..
            } => {
                transaction.name.eq_ignore_ascii_case(value)
                    && transaction.trans_type == *trans_type
            }
            ClassificationRule::Contains {
                value, trans_type, ..
            } => {
                transaction
                    .name
                    .to_lowercase()
                    .contains(&value.to_lowercase())
                    && transaction.trans_type == *trans_type
            }
        }
    }

    pub fn kind(&self) -> TransactionKind {
        match self {
            ClassificationRule::Exact { kind, .. } => *kind,
            ClassificationRule::Contains { kind, .. } => *kind,
        }
    }
}
