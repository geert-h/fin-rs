pub mod transactions;

use ratatui::Frame;

use crate::app::App;

pub fn render(frame: &mut Frame, app: &mut App) {
    transactions::render(frame, app);
}
