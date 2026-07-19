use serde::{Deserialize, Serialize};

use crate::transaction::model::{Transaction, TransactionType};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Category {
    Income,
    FixedExpense,
    VariableExpense,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TransactionKind {
    // Income
    Scholarship,
    Salary,
    HealthcareAllowance,
    RentAllowance,
    OrphansPension,
    OtherIncome,

    // Fixed expenses
    TuitionFees,
    HealthInsurance,
    OtherInsurance,
    Rent,
    Laundry,
    Phone,
    Subscriptions,
    Investments,
    Sports,

    // Variable expenses
    Groceries,
    Clothing,
    Hairdresser,
    Entertainment,
    SnacksAndCoffee,
    PersonalCare,
    WorkLunch,
    PublicTransport,
    HardwareAndSoftware,
    Vacation,
    OtherVariableExpense,
}

impl TransactionKind {
    pub const fn category(self) -> Category {
        match self {
            Self::Scholarship
            | Self::Salary
            | Self::HealthcareAllowance
            | Self::RentAllowance
            | Self::OrphansPension
            | Self::OtherIncome => Category::Income,

            Self::TuitionFees
            | Self::HealthInsurance
            | Self::OtherInsurance
            | Self::Rent
            | Self::Laundry
            | Self::Phone
            | Self::Subscriptions
            | Self::Investments
            | Self::Sports => Category::FixedExpense,

            Self::Groceries
            | Self::Clothing
            | Self::Hairdresser
            | Self::Entertainment
            | Self::SnacksAndCoffee
            | Self::PersonalCare
            | Self::WorkLunch
            | Self::PublicTransport
            | Self::HardwareAndSoftware
            | Self::Vacation
            | Self::OtherVariableExpense => Category::VariableExpense,
        }
    }
}

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
