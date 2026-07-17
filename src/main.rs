use std::{
    env::args_os,
    error::Error,
    ffi::OsString,
    path::{Path, PathBuf},
};

use crate::parser::{TransactionsParser, ing::IngParser};

mod parser;
mod store;
mod transaction;

fn main() -> Result<(), Box<dyn Error>> {
    let path: PathBuf = get_first_arg()?.into();
    let path = Path::new(&path);

    let new_transactions = IngParser::parse(path)?;

    let storage_path = "transactions/store.jsonl".to_string();
    let storage_path = Path::new(&storage_path);
    store::store(storage_path, new_transactions)?;

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
