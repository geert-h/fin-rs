use std::{
    env::args_os,
    error::Error,
    ffi::OsString,
    path::{Path, PathBuf},
};

use crate::{
    classification::Classifier,
    parser::{TransactionsParser, ing::IngParser},
    store::TransactionStore,
};

mod classification;
mod parser;
mod store;
mod transaction;

fn main() -> Result<(), Box<dyn Error>> {
    let path: PathBuf = get_first_arg()?.into();
    let path = Path::new(&path);

    // Parse the new transactions
    let mut transactions = IngParser::parse(path)?;

    // Classify the transactions based on defined rules
    let classifier = Classifier::new(Path::new("transactions/classification.jsonl"))?;
    for t in &mut transactions {
        classifier.classify_transaction(t);
        if let Some(kind) = t.kind {
            println!("{} -> {:?} / {:?}", t.name, kind.category(), kind,);
        }
    }

    let store = TransactionStore::new("transactions/store.jsonl");
    // Write them to file
    store.store(transactions)?;

    Ok(())
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
