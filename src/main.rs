use std::collections::HashMap;
use std::io;

use chrono::{Datelike, Local, NaiveDate, Weekday};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

struct App {
    year: i32,
    month: u32,
    today: NaiveDate,
    holidays: HashMap<NaiveDate, String>,
    should_quit: bool,
}

impl App {
    fn new(holidays: HashMap<NaiveDate, String>) -> Self {
        let today = Local::now().date_naive();
        Self {
            year: today.year(),
            month: today.month(),
            today,
            holidays,
            should_quit: false,
        }
    }

    fn prev_month(&mut self) {
        if self.month == 1 {
            self.month = 12;
            self.year -= 1;
        } else {
            self.month -= 1;
        }
    }

    fn next_month(&mut self) {
        if self.month == 12 {
            self.month = 1;
            self.year += 1;
        } else {
            self.month += 1;
        }
    }

    fn first_day_of_month(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year, self.month, 1).unwrap()
    }

    fn days_in_month(&self) -> u32 {
        let next = if self.month == 12 {
            NaiveDate::from_ymd_opt(self.year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(self.year, self.month + 1, 1)
        };
        next.unwrap()
            .signed_duration_since(self.first_day_of_month())
            .num_days() as u32
    }

    /// Returns the column index (0=Sun, 1=Mon, ..., 6=Sat) for a weekday.
    fn weekday_col(wd: Weekday) -> usize {
        match wd {
            Weekday::Sun => 0,
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
        }
    }

    fn holidays_this_month(&self) -> Vec<(u32, &str)> {
        let mut list: Vec<(u32, &str)> = Vec::new();
        for day in 1..=self.days_in_month() {
            if let Some(date) = NaiveDate::from_ymd_opt(self.year, self.month, day) {
                if let Some(name) = self.holidays.get(&date) {
                    list.push((day, name.as_str()));
                }
            }
        }
        list
    }
}

fn fetch_holidays() -> HashMap<NaiveDate, String> {
    let url = "https://holidays-jp.github.io/api/v1/date.json";
    let resp = match reqwest::blocking::get(url) {
        Ok(r) => r,
        Err(_) => return HashMap::new(),
    };
    let map: HashMap<String, String> = match resp.json() {
        Ok(m) => m,
        Err(_) => return HashMap::new(),
    };
    map.into_iter()
        .filter_map(|(k, v)| {
            NaiveDate::parse_from_str(&k, "%Y-%m-%d")
                .ok()
                .map(|d| (d, v))
        })
        .collect()
}

fn draw(frame: &mut Frame, app: &App) {
    let holiday_count = app.holidays_this_month().len().max(1); // 最低1行（"祝日なし"）
    let footer_height = (holiday_count + 1 + 2) as u16; // 祝日 + ヘルプ行 + 枠線
    let chunks = Layout::vertical([
        Constraint::Length(3),            // title
        Constraint::Min(8),              // calendar
        Constraint::Length(footer_height), // footer
    ])
    .split(frame.area());

    render_title(frame, app, chunks[0]);
    render_calendar(frame, app, chunks[1]);
    render_footer(frame, app, chunks[2]);
}

fn render_title(frame: &mut Frame, app: &App, area: Rect) {
    let title = format!("◀  {}年 {}月  ▶", app.year, app.month);
    let paragraph = Paragraph::new(Line::from(Span::styled(
        title,
        Style::default().add_modifier(Modifier::BOLD),
    )))
    .alignment(ratatui::layout::Alignment::Center)
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(paragraph, area);
}

fn render_calendar(frame: &mut Frame, app: &App, area: Rect) {
    let header_spans: Vec<Span> = ["  日", "  月", "  火", "  水", "  木", "  金", "  土"]
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let style = match i {
                0 => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                6 => Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
                _ => Style::default().add_modifier(Modifier::BOLD),
            };
            // 全角1文字=端末2カラム。"  日" でスペース2+全角2=4カラム。
            // format!で再パディングすると文字数基準でずれるのでそのまま使う。
            Span::styled(*label, style)
        })
        .collect();
    let mut lines: Vec<Line> = vec![Line::from(header_spans)];

    let first = app.first_day_of_month();
    let start_col = App::weekday_col(first.weekday());
    let days = app.days_in_month();

    let mut row_spans: Vec<Span> = Vec::new();
    // Fill leading blanks
    for _ in 0..start_col {
        row_spans.push(Span::raw("    "));
    }

    let mut col = start_col;
    for day in 1..=days {
        let date = NaiveDate::from_ymd_opt(app.year, app.month, day).unwrap();
        let is_holiday = app.holidays.contains_key(&date);
        let is_today = date == app.today;
        let is_sunday = col == 0;
        let is_saturday = col == 6;

        let label = format!("{: >4}", day);

        let style = if is_today {
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else if is_holiday || is_sunday {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else if is_saturday {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        row_spans.push(Span::styled(label, style));
        col += 1;

        if col == 7 {
            lines.push(Line::from(std::mem::take(&mut row_spans)));
            col = 0;
        }
    }
    if !row_spans.is_empty() {
        lines.push(Line::from(row_spans));
    }

    let paragraph = Paragraph::new(lines).block(Block::default().borders(Borders::ALL));
    frame.render_widget(paragraph, area);
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    let holidays = app.holidays_this_month();
    if holidays.is_empty() {
        lines.push(Line::from(Span::styled(
            "  祝日なし",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (day, name) in &holidays {
            lines.push(Line::from(Span::styled(
                format!("  {}/{}  {}", app.month, day, name),
                Style::default().fg(Color::Red),
            )));
        }
    }

    lines.push(Line::from(Span::styled(
        "  h/← prev   l/→ next   q/Esc quit",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines).block(Block::default().borders(Borders::ALL));
    frame.render_widget(paragraph, area);
}

fn handle_events(app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(());
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                KeyCode::Char('h') | KeyCode::Left => app.prev_month(),
                KeyCode::Char('l') | KeyCode::Right => app.next_month(),
                _ => {}
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let holidays = fetch_holidays();
    let mut app = App::new(holidays);

    // Setup terminal
    terminal::enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal =
        ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(io::stdout()))?;

    // Main loop
    loop {
        terminal.draw(|frame| draw(frame, &app))?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};

    fn app_with_holidays(year: i32, month: u32, holidays: Vec<(&str, &str)>) -> App {
        let map: HashMap<NaiveDate, String> = holidays
            .into_iter()
            .filter_map(|(date, name)| {
                NaiveDate::parse_from_str(date, "%Y-%m-%d")
                    .ok()
                    .map(|d| (d, name.to_string()))
            })
            .collect();
        App {
            year,
            month,
            today: NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            holidays: map,
            should_quit: false,
        }
    }

    #[test]
    fn prev_month_wraps_year() {
        let mut app = app_with_holidays(2026, 1, vec![]);
        app.prev_month();
        assert_eq!((app.year, app.month), (2025, 12));
    }

    #[test]
    fn next_month_wraps_year() {
        let mut app = app_with_holidays(2026, 12, vec![]);
        app.next_month();
        assert_eq!((app.year, app.month), (2027, 1));
    }

    #[test]
    fn days_in_february_leap_year() {
        let app = app_with_holidays(2028, 2, vec![]);
        assert_eq!(app.days_in_month(), 29);
    }

    #[test]
    fn days_in_february_non_leap() {
        let app = app_with_holidays(2026, 2, vec![]);
        assert_eq!(app.days_in_month(), 28);
    }

    #[test]
    fn holidays_this_month_filters_correctly() {
        let app = app_with_holidays(
            2026,
            5,
            vec![
                ("2026-05-03", "憲法記念日"),
                ("2026-05-04", "みどりの日"),
                ("2026-05-05", "こどもの日"),
                ("2026-05-06", "憲法記念日 振替休日"),
                ("2026-03-20", "春分の日"), // 別の月 - 含まれないはず
            ],
        );
        let holidays = app.holidays_this_month();
        assert_eq!(holidays.len(), 4);
        assert_eq!(holidays[0], (3, "憲法記念日"));
        assert_eq!(holidays[3], (6, "憲法記念日 振替休日"));
    }

    #[test]
    fn footer_shows_all_holidays_in_may() {
        let app = app_with_holidays(
            2026,
            5,
            vec![
                ("2026-05-03", "憲法記念日"),
                ("2026-05-04", "みどりの日"),
                ("2026-05-05", "こどもの日"),
                ("2026-05-06", "憲法記念日 振替休日"),
            ],
        );
        let mut terminal = Terminal::new(TestBackend::new(40, 20)).unwrap();
        terminal.draw(|frame| draw(frame, &app)).unwrap();
        let buf = terminal.backend().buffer().clone();

        // バッファ全体から各祝日名を検索
        // TestBackendは全角文字を「本体+空セル」で表現するため、
        // 空白を除去してから部分一致で検証する
        let content: String = (0..buf.area.height)
            .map(|y| {
                (0..buf.area.width)
                    .map(|x| buf.cell((x, y)).unwrap().symbol().to_string())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        let compact: String = content.chars().filter(|c| !c.is_whitespace()).collect();

        assert!(compact.contains("憲法記念日"), "憲法記念日が見つからない");
        assert!(compact.contains("みどりの日"), "みどりの日が見つからない");
        assert!(compact.contains("こどもの日"), "こどもの日が見つからない");
        assert!(
            compact.contains("振替休日"),
            "振替休日が見つからない:\n{content}"
        );
    }
}
