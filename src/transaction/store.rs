use crate::transaction::model::Transaction;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

pub struct TransactionStore {
    path: PathBuf,
}

impl TransactionStore {
    pub fn new(path: impl Into<PathBuf>) -> TransactionStore {
        Self { path: path.into() }
    }

    pub fn read_all(&self) -> Result<Vec<Transaction>, Box<dyn Error>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.path)?;

        let reader = BufReader::new(file);

        let mut transactions = Vec::new();

        for (index, line) in reader.lines().enumerate() {
            let line = line?;

            if line.trim().is_empty() {
                continue;
            }

            let transaction = serde_json::from_str::<Transaction>(&line).map_err(|error| {
                format!(
                    "invalid transaction in {} on line {}: {}",
                    self.path.display(),
                    index + 1,
                    error
                )
            })?;

            transactions.push(transaction);
        }

        Ok(transactions)
    }

    pub fn append(&self, transactions: &[Transaction]) -> Result<(), Box<dyn Error>> {
        if transactions.is_empty() {
            return Ok(());
        }

        self.ensure_parent_directory()?;

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        let mut writer = BufWriter::new(file);

        for transaction in transactions {
            serde_json::to_writer(&mut writer, transaction)?;

            writer.write_all(b"\n")?;
        }

        writer.flush()?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn write_all(&self, transactions: &[Transaction]) -> Result<(), Box<dyn Error>> {
        self.ensure_parent_directory()?;

        let file = File::create(&self.path)?;

        let mut writer = BufWriter::new(file);

        for transaction in transactions {
            serde_json::to_writer(&mut writer, transaction)?;

            writer.write_all(b"\n")?;
        }

        writer.flush()?;

        Ok(())
    }

    fn ensure_parent_directory(&self) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = self.path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }

        Ok(())
    }
}
