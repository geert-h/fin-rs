use std::{error::Error, path::Path};

use crate::{
    classification::handler::ClassificationHandler,
    transaction::{handler::Transactionhandler, parser::TransactionsParser},
};

#[derive(Debug)]
pub struct ImportResult {
    pub parsed: usize,
    pub added: usize,
    pub duplicates: usize,
    pub classified: usize,
    pub unclassified: usize,
}

pub struct ImportHandler<'a> {
    transaction_handler: &'a Transactionhandler,
    classification_handler: &'a ClassificationHandler,
}

impl<'a> ImportHandler<'a> {
    pub fn new(
        transaction_handler: &'a Transactionhandler,
        classification_handler: &'a ClassificationHandler,
    ) -> Self {
        Self {
            transaction_handler,
            classification_handler,
        }
    }

    pub fn import<P>(&self, path: &Path) -> Result<ImportResult, Box<dyn Error>>
    where
        P: TransactionsParser,
    {
        let transactions = P::parse(path)?;
        let parsed = transactions.len();

        // Remove transactions that have already been stored.
        let mut new_transactions = self.transaction_handler.filter_new(transactions)?;

        let added = new_transactions.len();
        let duplicates = parsed - added;

        // Only classify transactions that will actually be stored.
        for transaction in &mut new_transactions {
            self.classification_handler
                .classify_transaction(transaction);
        }

        let classified = new_transactions
            .iter()
            .filter(|transaction| transaction.kind.is_some())
            .count();

        let unclassified = added - classified;

        self.transaction_handler.append(&new_transactions)?;

        Ok(ImportResult {
            parsed,
            added,
            duplicates,
            classified,
            unclassified,
        })
    }
}

