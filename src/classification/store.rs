use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use crate::classification::rule::ClassificationRule;

/// Reads the classification rules from a certain file
pub(crate) fn read_rules(path: &Path) -> Result<Vec<ClassificationRule>, Box<dyn Error>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path)?;
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
pub(crate) fn write_rules(
    path: &Path,
    rules: Vec<ClassificationRule>,
) -> Result<(), Box<dyn Error>> {
    if rules.is_empty() {
        return Ok(());
    }

    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);

    for t in rules {
        serde_json::to_writer(&mut writer, &t)?;
        writer.write_all(b"\n")?;
    }

    writer.flush()?;

    Ok(())
}
