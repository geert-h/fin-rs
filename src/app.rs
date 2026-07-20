use ratatui::widgets::TableState;

use crate::transaction::model::Transaction;

pub struct App {
    pub transactions: Vec<Transaction>,
    pub transaction_table_state: TableState,
    pub should_quit: bool,
}

impl App {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        let mut transaction_table_state = TableState::default();

        if !transactions.is_empty() {
            transaction_table_state.select(Some(0));
        }

        Self {
            transactions,
            transaction_table_state,
            should_quit: false,
        }
    }

    pub fn next_transaction(&mut self) {
        if self.transactions.is_empty() {
            return;
        }

        let current = self.transaction_table_state.selected().unwrap_or(0);
        let next = if current >= self.transactions.len() - 1 {
            0
        } else {
            current + 1
        };

        self.transaction_table_state.select(Some(next));
    }

    pub fn previous_transaction(&mut self) {
        if self.transactions.is_empty() {
            return;
        }

        let current = self.transaction_table_state.selected().unwrap_or(0);
        let previous = if current == 0 {
            self.transactions.len() - 1
        } else {
            current - 1
        };

        self.transaction_table_state.select(Some(previous));
    }

    pub fn selected_transaction(&self) -> Option<&Transaction> {
        let index = self.transaction_table_state.selected()?;
        self.transactions.get(index)
    }

    pub fn selected_transaction_mut(&mut self) -> Option<&mut Transaction> {
        let index = self.transaction_table_state.selected()?;
        self.transactions.get_mut(index)
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
