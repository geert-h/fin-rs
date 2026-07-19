use std::{env::args_os, error::Error, ffi::OsString, path::PathBuf};

use crate::{
    classification::handler::ClassificationHandler,
    import::handler::ImportHandler,
    transaction::{
        handler::Transactionhandler,
        parser::{TransactionsParser, ing::IngParser},
        store::TransactionStore,
    },
};

mod classification;
mod import;
mod transaction;

fn main() -> Result<(), Box<dyn Error>> {
    let import_path: PathBuf = get_first_arg()?.into();

    let transaction_store = TransactionStore::new("transactions/store.jsonl");

    let transaction_handler = Transactionhandler::new(transaction_store);

    let classification_handler =
        ClassificationHandler::from_file("transactions/classification.jsonl".into())?;

    let import_handler = ImportHandler::new(&transaction_handler, &classification_handler);

    let result = import_handler.import::<IngParser>(&import_path)?;

    println!("parsed: {}", result.parsed);
    println!("added: {}", result.added);
    println!("duplicates: {}", result.duplicates);
    println!("classified: {}", result.classified);
    println!("unclassified: {}", result.unclassified);

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
