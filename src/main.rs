mod app;
mod ui;

use std::io;

use crossterm::ExecutableCommand;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};

fn handle_events(app: &mut app::App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))?
        && let Event::Key(key) = event::read()?
    {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
            KeyCode::Char('h') | KeyCode::Left => app.prev_month(),
            KeyCode::Char('l') | KeyCode::Right => app.next_month(),
            KeyCode::Char('t') => app.go_today(),
            _ => {}
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let holidays = app::fetch_holidays();
    let mut app = app::App::new(holidays);

    // Setup terminal
    terminal::enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal =
        ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(io::stdout()))?;

    // Main loop
    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;
        handle_events(&mut app)?;
        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    terminal::disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
