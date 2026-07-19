use std::{collections::HashSet, error::Error};

use uuid::Uuid;

use crate::transaction::{model::Transaction, store::TransactionStore};

pub struct Transactionhandler {
    store: TransactionStore,
}

impl Transactionhandler {
    pub fn new(store: TransactionStore) -> Self {
        Self { store }
    }

    #[allow(dead_code)]
    pub fn all(&self) -> Result<Vec<Transaction>, Box<dyn Error>> {
        self.store.read_all()
    }

    pub fn filter_new(
        &self,
        candidates: Vec<Transaction>,
    ) -> Result<Vec<Transaction>, Box<dyn Error>> {
        let stored = self.store.read_all()?;

        let mut known_ids: HashSet<Uuid> =
            stored.iter().map(|transaction| transaction.id).collect();

        let new_transactions = candidates
            .into_iter()
            .filter(|transaction| known_ids.insert(transaction.id))
            .collect();

        Ok(new_transactions)
    }

    pub fn append(&self, transactions: &[Transaction]) -> Result<(), Box<dyn Error>> {
        self.store.append(transactions)
    }

    #[allow(dead_code)]
    pub fn replace_all(&self, transactions: &[Transaction]) -> Result<(), Box<dyn Error>> {
        self.store.write_all(transactions)
    }
}
