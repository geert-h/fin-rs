use std::{env::args_os, error::Error, ffi::OsString, path::Path};

use crate::parser::{TransactionsParser, ing::IngParser};

mod parser;
mod transaction;

fn main() -> Result<(), Box<dyn Error>> {
    let path = get_first_arg()?;
    let path = Path::new(&path);

    let transactions = IngParser::parse(path)?;

    for t in transactions.iter().take(2) {
        println!(
            "{} {} {} {} {}",
            t.id, t.amount, t.date, t.name, t.trans_type
        );
    }

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
