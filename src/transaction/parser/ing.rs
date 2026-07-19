use std::{error::Error, path::Path, str::FromStr};

use chrono::{Local, NaiveDate, NaiveDateTime, TimeZone};
use serde::{Deserialize, Deserializer};
use uuid::{Uuid, uuid};

use crate::{
    TransactionsParser,
    transaction::model::{Transaction, TransactionType},
};

pub struct IngParser;

impl TransactionsParser for IngParser {
    fn parse(path: &Path) -> Result<Vec<Transaction>, Box<dyn Error>> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b';')
            .from_path(path)?;
        let mut transactions = vec![];

        let mut cur_date = None;
        let mut index: usize = 0;

        for result in reader.deserialize() {
            let record: IngTransactionRow = result?;

            let date = NaiveDate::parse_from_str(&record.date, "%Y%m%d")?
                .and_hms_opt(0, 0, 0)
                .ok_or("invalid transaction time")?;

            if cur_date.is_some_and(|d| d == date) {
                index += 1;
            } else {
                index = 0;
                cur_date = Some(date);
            }

            let id = transaction_id(date, index);

            let date = Local
                .from_local_datetime(&date)
                .single()
                .ok_or("ambiguous or invalid local date")?;

            let trans_type = TransactionType::from_str(&record.debit_credit)?;

            transactions.push(Transaction {
                id,
                date,
                name: record.name_description,
                trans_type,
                amount: record.amount,
                kind: None,
            })
        }

        Ok(transactions)
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct IngTransactionRow {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Name / Description")]
    name_description: String,
    #[serde(rename = "Account")]
    account: String,
    #[serde(rename = "Counterparty")]
    counterparty: String,
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Debit/credit")]
    debit_credit: String,
    #[serde(rename = "Amount (EUR)", deserialize_with = "deserialize_amount")]
    amount: i64,
    #[serde(rename = "Transaction type")]
    transaction_type: String,
    #[serde(rename = "Notifications")]
    notifications: String,
    #[serde(rename = "Resulting balance")]
    resulting_balance: String,
    #[serde(rename = "Tag")]
    tag: String,
}

/// Deserializes the amount to `i64`
fn deserialize_amount<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    value
        .trim()
        .replace(".", "")
        .replace(",", "")
        .parse::<i64>()
        .map_err(serde::de::Error::custom)
}

/// returns a unique `Uuid` for a date and daily_index
///
/// This is a bit of a rough way to go about it, but sadly the ING API doesn't
/// give any identifiers. Luckily the order in which entries in the CSV file are
/// returned is seemingly constant, so we leverage this to generate V5
/// identifiers.
fn transaction_id(date: NaiveDateTime, daily_index: usize) -> Uuid {
    let identity = format!("{}{}", date.format("%Y%m%d"), daily_index);

    Uuid::new_v5(
        &uuid!("ff1b1068-79e0-47d0-b613-21db3843c058"),
        identity.as_bytes(),
    )
}
