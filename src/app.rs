use std::collections::HashMap;

use chrono::{Datelike, Local, NaiveDate, Weekday};

pub struct App {
    pub year: i32,
    pub month: u32,
    pub today: NaiveDate,
    pub holidays: HashMap<NaiveDate, String>,
    pub should_quit: bool,
}

impl App {
    pub fn new(holidays: HashMap<NaiveDate, String>) -> Self {
        let today = Local::now().date_naive();
        Self {
            year: today.year(),
            month: today.month(),
            today,
            holidays,
            should_quit: false,
        }
    }

    pub fn prev_month(&mut self) {
        if self.month == 1 {
            self.month = 12;
            self.year -= 1;
        } else {
            self.month -= 1;
        }
    }

    pub fn next_month(&mut self) {
        if self.month == 12 {
            self.month = 1;
            self.year += 1;
        } else {
            self.month += 1;
        }
    }

    pub fn go_today(&mut self) {
        self.year = self.today.year();
        self.month = self.today.month();
    }

    pub fn first_day_of_month(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year, self.month, 1).unwrap()
    }

    pub fn days_in_month(&self) -> u32 {
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
    pub fn weekday_col(wd: Weekday) -> usize {
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

    pub fn holidays_this_month(&self) -> Vec<(u32, &str)> {
        let mut list: Vec<(u32, &str)> = Vec::new();
        for day in 1..=self.days_in_month() {
            if let Some(date) = NaiveDate::from_ymd_opt(self.year, self.month, day)
                && let Some(name) = self.holidays.get(&date)
            {
                list.push((day, name.as_str()));
            }
        }
        list
    }
}

pub fn fetch_holidays() -> HashMap<NaiveDate, String> {
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn app_with_holidays(year: i32, month: u32, holidays: Vec<(&str, &str)>) -> App {
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
    fn go_today_returns_to_current_month() {
        let mut app = app_with_holidays(2026, 3, vec![]);
        // 別の月に移動
        app.year = 2025;
        app.month = 8;
        // tで今日に戻る
        app.go_today();
        assert_eq!((app.year, app.month), (2026, 3));
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
}
