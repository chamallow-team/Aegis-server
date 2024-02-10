use std::collections::HashMap;
use chrono::{Datelike, DateTime, Timelike, Utc};
use crate::log::{Log, LogType};

#[derive(Debug, Clone, Default)]
pub(crate) struct Fmt {
    styles: Style
}

impl Fmt {
    /// Format a log with the current style.
    pub(crate) fn format(&self, log: &Log) -> String {
        self.styles.format(log)
    }

    /// Get the current style.
    pub(crate) fn get_style(&self) -> &Style {
        &self.styles
    }

    /// Set the style.
    pub(crate) fn set_style(&mut self, style: Style) {
        self.styles = style;
    }
}

#[derive(Clone, Debug, Default)]
pub enum DateStyle {
    #[default]
    HourMinuteSecond,
    HourMinute,
    Hour,
    DayMonthYear,
    Full
}

#[derive(Debug, Clone)]
pub struct Style {
    /// The colors for each log type.
    ///
    /// The color code is the ANSI color code.
    colors: HashMap<LogType, u8>,
    pattern: String,
    no_route: String,
    date_style: DateStyle,
    print_route_when_none: bool
}

impl Style {
    /// Define a new color for a log type.
    pub fn set_color(&mut self, log_type: LogType, color: u8) {
        self.colors.insert(log_type, color);
    }

    /// Remove a color for a log type.
    pub fn remove_color(&mut self, log_type: LogType) {
        self.colors.remove(&log_type);
    }

    /// Get the color for a log type.
    pub fn get_color(&self, log_type: &LogType) -> Option<&u8> {
        self.colors.get(log_type)
    }

    /// Set the pattern for formatting logs.
    pub fn set_pattern(&mut self, pattern: impl ToString) {
        self.pattern = pattern.to_string();
    }

    /// Get the pattern for formatting logs.
    pub fn set_no_route(&mut self, no_route: impl ToString) {
        self.no_route = no_route.to_string();
    }

    /// Get the pattern for formatting logs.
    pub fn set_date_style(&mut self, date_style: DateStyle) {
        self.date_style = date_style;
    }

    /// Get the pattern for formatting logs.
    pub fn get_date_style(&self) -> &DateStyle {
        &self.date_style
    }

    /// Get the pattern for formatting logs.
    pub fn set_print_route_when_none(&mut self, print_route_when_none: bool) {
        self.print_route_when_none = print_route_when_none;
    }

    /// Get the pattern for formatting logs.
    pub fn get_print_route_when_none(&self) -> bool {
        self.print_route_when_none
    }
}

impl Default for Style {
    fn default() -> Self {
        let mut colors = HashMap::new();
        colors.insert(LogType::Trace, 37);
        colors.insert(LogType::Debug, 37);
        colors.insert(LogType::Info, 34);
        colors.insert(LogType::Warn, 33);
        colors.insert(LogType::Error, 31);
        colors.insert(LogType::Panic, 31);

        Self {
            colors,
            pattern: DEFAULT_PATTERN.to_string(),
            no_route: DEFAULT_NO_ROUTE.to_string(),
            print_route_when_none: false,
            date_style: DateStyle::default()
        }
    }
}

const REGEX_PATTERN: &str = r"\{(?P<type>\w+)\}";

/// The default pattern for formatting logs.
pub const DEFAULT_PATTERN: &str = "[{date}] \x1b[37m[{type}]\x1b[0m {route} {c}{text}{sc}";

pub const DEFAULT_NO_ROUTE: &str = "::";

impl Style {
    /// Format a log with the current style.
    pub(crate) fn format(&self, log: &Log) -> String {
        // use internal_format to format the log
        if let Some(color) = self.colors.get(&log.log_type()) {
            self.internal_format(log, *color)
        } else {
            self.internal_format(log, 0)
        }
    }

    /// Format a log with the current style.
    ///
    /// This function use a regex to replace the pattern with the log's data.
    fn internal_format(&self, log: &Log, color: u8) -> String {
        let mut formatted = self.pattern.clone();

        let re = regex::Regex::new(REGEX_PATTERN).expect("Well, this is a problem");

        for cap in re.captures_iter(&self.pattern) {
            let matched = cap.get(0).unwrap().as_str();
            let n = cap.name("type").map(|s| s.as_str()).unwrap_or_default();
            match n {
                "text" => formatted = formatted.replace(matched, log.text()),
                "route" => {
                    match log.route() {
                        Some(route) => formatted = formatted.replace(matched, route),
                        None if self.print_route_when_none => formatted = formatted.replace(matched, self.no_route.as_str()),
                        _ => formatted = formatted.replace(matched, "")
                    }
                },
                "date" => formatted = formatted.replace(matched, self.format_date(&log.date()).as_str()),
                "type" =>  formatted = formatted.replace(matched, &log.log_type().to_string()),
                // "c" means color
                "c" if color != 0 => formatted = formatted.replace(matched, &format!("\x1b[{}m", color)),
                "c" => formatted = formatted.replace(matched, ""),
                // "sc" means style close
                "sc" => formatted = formatted.replace(matched, "\x1b[0m"),
                _ => {}
            }
        }

        formatted
    }

    /// Format a date with the current date style.
    fn format_date(&self, date: &DateTime<Utc>) -> String {
        let mut s = String::new();
        match self.date_style {
            DateStyle::HourMinuteSecond => s.push_str(&format!("{:02}:{:02}:{:02}", date.hour(), date.minute(), date.second())),
            DateStyle::HourMinute => s.push_str(&format!("{:02}:{:02}", date.hour(), date.minute())),
            DateStyle::Hour => s.push_str(&format!("{:02}", date.hour())),
            DateStyle::DayMonthYear => s.push_str(&format!("{:02}/{:02}/{:04}", date.day(), date.month(), date.year())),
            DateStyle::Full => s.push_str(&format!("{:02}/{:02}/{:04} {:02}:{:02}:{:02}", date.day(), date.month(), date.year(), date.hour(), date.minute(), date.second()))
        }

        s
    }
}