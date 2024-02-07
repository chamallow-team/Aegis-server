use std::collections::HashMap;
use crate::log::{Log, LogType};

#[derive(Debug, Clone, Default)]
pub(crate) struct Fmt {
    styles: Style
}

impl Fmt {
    pub(crate) fn format(&self, log: &Log) -> String {
        self.styles.format(log)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Style {
    /// The colors for each log type.
    ///
    /// The color code is the ANSI color code.
    colors: HashMap<LogType, u8>,
    pattern: String,
    no_route: String,
    print_route_when_none: bool
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
            print_route_when_none: false
        }
    }
}

const REGEX_PATTERN: &str = r"\{(?P<type>\w+)\}";

/// The default pattern for formatting logs.
const DEFAULT_PATTERN: &str = "[{date}] \x1b[37m[{type}]\x1b[0m {route} {c}{text}{sc}";

const DEFAULT_NO_ROUTE: &str = "::";

impl Style {
    pub(crate) fn format(&self, log: &Log) -> String {
        // use internal_format to format the log
        if let Some(color) = self.colors.get(&log.log_type()) {
            self.internal_format(log, *color)
        } else {
            self.internal_format(log, 0)
        }
    }

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
                "date" => formatted = formatted.replace(matched, &log.date().to_rfc2822()),
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
}