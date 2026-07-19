use crate::{
    classification::{model::ClassificationRule, store::ClassificationStore},
    transaction::model::Transaction,
};
use std::{error::Error, path::PathBuf};

pub struct ClassificationHandler {
    store: ClassificationStore,
    rules: Vec<ClassificationRule>,
}

impl ClassificationHandler {
    pub fn from_file(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let store = ClassificationStore::new(path);
        Ok(Self {
            rules: store.read_rules()?,
            store,
        })
    }

    #[allow(dead_code)]
    pub fn add_rule(&mut self, rule: ClassificationRule) {
        self.rules.push(rule);
    }

    #[allow(dead_code)]
    pub fn to_file(&self) -> Result<(), Box<dyn Error>> {
        self.store.write_rules(self.rules.clone())
    }

    pub fn classify_transaction(&self, t: &mut Transaction) {
        let matching_rules: Vec<_> = self.rules.iter().filter(|rule| rule.matches(&t)).collect();

        t.kind = matching_rules.first().map(|rule| rule.kind());
    }
}
