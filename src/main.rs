mod app;
mod ui;
mod update;

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

fn run_tui() -> io::Result<()> {
    let holidays = app::fetch_holidays();
    let mut app = app::App::new(holidays);

    terminal::enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal =
        ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(io::stdout()))?;

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;
        handle_events(&mut app)?;
        if app.should_quit {
            break;
        }
    }

    terminal::disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(String::as_str) {
        None => {
            if let Err(e) = run_tui() {
                eprintln!("エラー: {e}");
                std::process::exit(1);
            }
        }
        Some("update") => update::run_update(),
        Some(cmd) => {
            eprintln!("不明なコマンド: {cmd}");
            eprintln!("使い方: jcal [update]");
            std::process::exit(1);
        }
    }
}
