use std::{error::Error, path::Path};

use serde::{Deserialize, Serialize};

use crate::{classification::rule::ClassificationRule, transaction::Transaction};

pub mod rule;
pub mod store;

pub struct Classifier {
    rules: Vec<ClassificationRule>,
}

impl Classifier {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            rules: store::read_rules(path)?,
        })
    }

    #[allow(dead_code)]
    pub fn add_rule(&mut self, rule: ClassificationRule) {
        self.rules.push(rule);
    }

    #[allow(dead_code)]
    pub fn to_file(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        store::write_rules(path, self.rules.clone())
    }

    pub fn classify_transaction(&self, t: &mut Transaction) {
        let matching_rules: Vec<_> = self.rules.iter().filter(|rule| rule.matches(&t)).collect();

        t.kind = matching_rules.first().map(|rule| rule.kind());
    }
}

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
