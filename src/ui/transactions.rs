use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::{app::App, transaction::model::Transaction};

pub fn render(frame: &mut Frame, app: &mut App) {
    let [table_area, help_area] =
        Layout::vertical([Constraint::Min(5), Constraint::Length(3)]).areas(frame.area());

    render_transaction_table(frame, table_area, app);
    render_help(frame, help_area);
}

fn render_transaction_table(frame: &mut Frame, area: Rect, app: &mut App) {
    let header = Row::new([
        Cell::from("Date"),
        Cell::from("Description"),
        Cell::from("Amount"),
        Cell::from("Classification"),
    ])
    .style(Style::default().add_modifier(Modifier::BOLD))
    .bottom_margin(1);

    let rows = app.transactions.iter().map(transaction_row);

    let widths = [
        Constraint::Length(12),
        Constraint::Min(30),
        Constraint::Length(14),
        Constraint::Length(24),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ")
        .block(
            Block::default()
                .title(format!(" Transactions ({}) ", app.transactions.len()))
                .borders(Borders::ALL),
        );

    frame.render_stateful_widget(table, area, &mut app.transaction_table_state);
}

fn transaction_row(transaction: &Transaction) -> Row<'static> {
    let date = transaction.date.format("%Y-%m-%d").to_string();

    let amount = format_amount(transaction.amount);

    let classification = transaction
        .kind
        .map(|kind| format!("{kind:?}"))
        .unwrap_or_else(|| "Unclassified".to_owned());

    Row::new([
        Cell::from(date),
        Cell::from(transaction.name.clone()),
        Cell::from(amount),
        Cell::from(classification),
    ])
}

fn format_amount(amount_in_cents: i64) -> String {
    let sign = if amount_in_cents < 0 { "-" } else { "" };

    let absolute = amount_in_cents.unsigned_abs();

    format!("{sign}€{}.{:02}", absolute / 100, absolute % 100,)
}

fn render_help(frame: &mut Frame, area: Rect) {
    let help = Paragraph::new(Line::from("↑/k previous  ↓/j next  q quit"))
        .block(Block::default().title(" Controls ").borders(Borders::ALL));

    frame.render_widget(help, area);
}
