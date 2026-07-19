use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use crate::classification::model::ClassificationRule;

pub struct ClassificationStore {
    path: PathBuf,
}

impl ClassificationStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Reads the classification rules from a certain file
    pub(crate) fn read_rules(&self) -> Result<Vec<ClassificationRule>, Box<dyn Error>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut rules = Vec::new();

        for (line_number, line) in reader.lines().enumerate() {
            let line = line?;

            if line.trim().is_empty() {
                continue;
            }

            let rule = serde_json::from_str::<ClassificationRule>(&line).map_err(|err| {
                format!(
                    "invalid classification rule on line {}: {}",
                    line_number + 1,
                    err
                )
            })?;

            rules.push(rule);
        }

        Ok(rules)
    }

    #[allow(dead_code)]
    /// Appends new classification rules to the rule file.
    ///
    /// Currently this is not used because the functionality for adding new rules is not there yet.
    pub(crate) fn write_rules(&self, rules: Vec<ClassificationRule>) -> Result<(), Box<dyn Error>> {
        if rules.is_empty() {
            return Ok(());
        }

        let file = File::create(&self.path)?;
        let mut writer = BufWriter::new(file);

        for t in rules {
            serde_json::to_writer(&mut writer, &t)?;
            writer.write_all(b"\n")?;
        }

        writer.flush()?;

        Ok(())
    }
}
