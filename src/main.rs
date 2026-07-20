mod app;
mod classification;
mod import;
mod transaction;
mod ui;

use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{app::App, transaction::store::TransactionStore};

fn main() -> Result<(), Box<dyn Error>> {
    let store = TransactionStore::new("transactions/store.jsonl");
    let transactions = store.read_all()?;

    let mut app = App::new(transactions);

    let mut terminal = initialize_terminal()?;
    let result = run_app(&mut terminal, &mut app);
    restore_terminal(&mut terminal)?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {
    while !app.should_quit {
        terminal.draw(|frame| ui::render(frame, app))?;

        if !event::poll(Duration::from_millis(100))? {
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };

        // Crossterm can report key press, repeat, and release events.
        // Only respond to presses and repeats.
        if key.kind == KeyEventKind::Release {
            continue;
        }

        match key.code {
            KeyCode::Char('q') => app.quit(),

            KeyCode::Down | KeyCode::Char('j') => {
                app.next_transaction();
            }

            KeyCode::Up | KeyCode::Char('k') => {
                app.previous_transaction();
            }

            _ => {}
        }
    }

    Ok(())
}

fn initialize_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;

    terminal.show_cursor()?;

    Ok(())
}
