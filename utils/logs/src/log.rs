use std::fmt::Display;

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LogType {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
    Panic
}

impl Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogType::Trace => write!(f, "TRACE"),
            LogType::Debug => write!(f, "DEBUG"),
            LogType::Info => write!(f, "INFO"),
            LogType::Warn => write!(f, "WARN"),
            LogType::Error => write!(f, "ERROR"),
            LogType::Panic => write!(f, "PANIC")
        }
    }
}

/// A log.
///
/// This struct is used to represent a log that can be written to a stream.
#[derive(Debug, Clone)]
pub struct Log {
    route: Option<String>,
    text: String,
    log_type: LogType,
    date: chrono::DateTime<chrono::Utc>
}

impl Log {
    /// Creates a new log.
    pub fn new<T: Into<Log>>(log: T) -> Self {
        log.into()
    }

    /// Returns the source of the log.
    pub fn route(&self) -> Option<&str> {
        self.route.as_deref()
    }

    /// Returns the text of the log.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the type of the log.
    pub fn log_type(&self) -> LogType {
        self.log_type
    }

    /// Returns the date of the log.
    pub fn date(&self) -> chrono::DateTime<chrono::Utc> {
        self.date
    }

    /// Sets the source of the log.
    pub fn set_route<T: Into<String>>(&mut self, source: T) {
        self.route = Some(source.into());
    }

    /// Sets the text of the log.
    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
    }
}

impl From<String> for Log {
    fn from(text: String) -> Self {
        Self { route: None, text, log_type: LogType::default(), date: chrono::Utc::now() }
    }
}

impl From<&str> for Log {
    fn from(text: &str) -> Self {
        Self { route: None, text: text.to_string(), log_type: LogType::default(), date: chrono::Utc::now() }
    }
}

impl From<(String, String)> for Log {
    fn from((source, text): (String, String)) -> Self {
        Self { route: Some(source), text, log_type: LogType::default(), date: chrono::Utc::now() }
    }
}

impl From<(String, &str)> for Log {
    fn from((source, text): (String, &str)) -> Self {
        Self { route: Some(source), text: text.to_string(), log_type: LogType::default(), date: chrono::Utc::now() }
    }
}

impl From<(&str, String)> for Log {
    fn from((source, text): (&str, String)) -> Self {
        Self { route: Some(source.to_string()), text, log_type: LogType::default(), date: chrono::Utc::now() }
    }
}

impl From<(&str, &str)> for Log {
    fn from((source, text): (&str, &str)) -> Self {
        Self { route: Some(source.to_string()), text: text.to_string(), log_type: LogType::default(), date: chrono::Utc::now() }
    }
}

impl From<LogType> for Log {
    fn from(log_type: LogType) -> Self {
        Self { route: None, text: String::new(), log_type, date: chrono::Utc::now() }
    }
}

impl From<(LogType, String)> for Log {
    fn from((log_type, text): (LogType, String)) -> Self {
        Self { route: None, text, log_type, date: chrono::Utc::now() }
    }
}

impl From<(LogType, &str)> for Log {
    fn from((log_type, text): (LogType, &str)) -> Self {
        Self { route: None, text: text.to_string(), log_type, date: chrono::Utc::now() }
    }
}

impl From<(LogType, String, String)> for Log {
    fn from((log_type, source, text): (LogType, String, String)) -> Self {
        Self { route: Some(source), text, log_type, date: chrono::Utc::now() }
    }
}

impl From<(LogType, String, &str)> for Log {
    fn from((log_type, source, text): (LogType, String, &str)) -> Self {
        Self { route: Some(source), text: text.to_string(), log_type, date: chrono::Utc::now() }
    }
}

impl From<(LogType, &str, String)> for Log {
    fn from((log_type, source, text): (LogType, &str, String)) -> Self {
        Self { route: Some(source.to_string()), text, log_type, date: chrono::Utc::now() }
    }
}

impl From<(LogType, &str, &str)> for Log {
    fn from((log_type, source, text): (LogType, &str, &str)) -> Self {
        Self { route: Some(source.to_string()), text: text.to_string(), log_type, date: chrono::Utc::now() }
    }
}
