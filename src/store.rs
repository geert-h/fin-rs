use std::{
    collections::HashSet,
    error::Error,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use uuid::Uuid;

use crate::transaction::Transaction;

pub struct TransactionStore {
    path: PathBuf,
}

impl TransactionStore {
    pub fn new(path: impl Into<PathBuf>) -> TransactionStore {
        Self { path: path.into() }
    }

    pub fn store(&self, transactions: Vec<Transaction>) -> Result<(), Box<dyn Error>> {
        let stored = self.read_transactions()?;
        let candidate_count = transactions.len();
        let new_transactions = prune_existing_transactions(stored, transactions);

        println!(
            "storing {} new transactions. {} were duplicate",
            new_transactions.len(),
            candidate_count - new_transactions.len(),
        );

        self.append_transactions(new_transactions)?;

        println!(
            "transactions succesfully written to {}",
            self.path.to_string_lossy()
        );

        Ok(())
    }

    fn read_transactions(&self) -> Result<Vec<Transaction>, Box<dyn Error>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut transactions = Vec::new();

        for (line_number, line) in reader.lines().enumerate() {
            let line = line?;

            if line.trim().is_empty() {
                continue;
            }

            let transaction = serde_json::from_str::<Transaction>(&line).map_err(|err| {
                format!("invalid transaction on line {}: {}", line_number + 1, err)
            })?;

            transactions.push(transaction);
        }

        Ok(transactions)
    }

    fn append_transactions(&self, transactions: Vec<Transaction>) -> Result<(), Box<dyn Error>> {
        if transactions.is_empty() {
            return Ok(());
        }

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        let mut writer = BufWriter::new(file);

        for t in transactions {
            serde_json::to_writer(&mut writer, &t)?;
            writer.write_all(b"\n")?;
        }

        writer.flush()?;

        Ok(())
    }
}

/// Takes the current stored transactions and the new transactions.
/// It returns all the new transactions which were not present stored Vec.
fn prune_existing_transactions(
    stored: Vec<Transaction>,
    new: Vec<Transaction>,
) -> Vec<Transaction> {
    let mut known_ids: HashSet<Uuid> = stored.iter().map(|t| t.id).collect();

    new.into_iter().filter(|t| known_ids.insert(t.id)).collect()
}
