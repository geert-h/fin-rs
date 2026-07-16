use std::{error::Error, path::Path};

use crate::transaction::Transaction;

pub mod ing;

pub trait TransactionsParser {
    fn parse(path: &Path) -> Result<Vec<Transaction>, Box<dyn Error>>;
}
