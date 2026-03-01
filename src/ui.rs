use chrono::{Datelike, NaiveDate};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let holidays = app.holidays_this_month();
    let footer_height = (holidays.len().max(1) + 1 + 2) as u16; // 祝日(最低1行) + ヘルプ行 + 枠線
    let chunks = Layout::vertical([
        Constraint::Length(3),             // title
        Constraint::Min(8),                // calendar
        Constraint::Length(footer_height), // footer
    ])
    .split(frame.area());

    render_title(frame, app, chunks[0]);
    render_calendar(frame, app, chunks[1]);
    render_footer(frame, app, &holidays, chunks[2]);
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

fn render_footer(frame: &mut Frame, app: &App, holidays: &[(u32, &str)], area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    if holidays.is_empty() {
        lines.push(Line::from(Span::styled(
            "  祝日なし",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (day, name) in holidays {
            lines.push(Line::from(Span::styled(
                format!("  {}/{}  {}", app.month, day, name),
                Style::default().fg(Color::Red),
            )));
        }
    }

    lines.push(Line::from(Span::styled(
        "  h/← prev   l/→ next   t today   q/Esc quit",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines).block(Block::default().borders(Borders::ALL));
    frame.render_widget(paragraph, area);
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    use crate::app::tests::app_with_holidays;

    use super::*;

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
