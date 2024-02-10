//! TODO: Open the rules api
//! TODO: Test everything above (shit)
pub mod rules;
pub mod log;
mod runner;
pub mod fmt;


use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::ops::Deref;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use smol::channel::Sender;
use smol::lock::RwLock;
use crate::log::Log;
use crate::rules::Rules;

/// The ID of a stream.
///
/// The global ID is `__global__`.
pub type StreamID = String;

const GLOBAL_STREAM_ID: &str = "__global__";

pub type Stream = Box<dyn Write + Send + Sync>;

pub struct Logger {
    /// A map of stream IDs to the streams themselves.
    ///
    /// This can allow for multiple streams to be written to at the same time.
    streams: Arc<RwLock<HashMap<StreamID, Stream>>>,
    /// The rules that apply to the logger.
    rules: Arc<RwLock<Rules>>,

    /// The sender that sends logs to the logger.
    sender: Sender<Log>,
    handle: JoinHandle<()>,

    immediate_flush: bool,
    formatter: Arc<RwLock<fmt::Fmt>>
}

impl Logger {
    /// Creates a new logger.
    ///
    /// By default, the logger will immediately flush the logs to the streams.
    #[allow(clippy::new_without_default)] // The default implementation is not wanted, as immediate_flush is set to true by default.
    pub fn new() -> Self {
        let (sender, receiver) = smol::channel::unbounded::<Log>();
        let streams = Arc::new(RwLock::new(HashMap::new()));
        let rules = Arc::new(RwLock::new(Rules::default()));
        let fmt = Arc::new(RwLock::new(fmt::Fmt::default()));

        // spawn a new task to handle the logs that are sent to the logger

        let streams_clone = streams.clone();
        let rules_clone = rules.clone();
        let fmt_clone = fmt.clone();

        let handle = thread::spawn(move || {
            smol::block_on(async {
                loop {
                    match receiver.recv().await {
                        Ok(l) => {
                            #[cfg(feature = "benchmark")]
                            let sb = std::time::Instant::now();

                            runner::run(
                                streams_clone.clone(),
                                rules_clone.clone(),
                                fmt_clone.clone(),
                                l
                            ).await;

                            #[cfg(feature = "benchmark")]
                            println!("Time to run: {}ms", sb.elapsed().as_millis());
                        },
                        Err(e) if !receiver.is_closed() => {
                            eprintln!("Error receiving log: {e:#?}, {}", e);
                            break;
                        }
                        // the sender was closed
                        Err(_) => break
                    }
                }
            })
        });

        Self {
            streams, rules, sender, handle,
            formatter: fmt,
            immediate_flush: true
        }
    }

    /// Get the style of the logger.
    pub async fn get_style(&self) -> fmt::Style {
        self.formatter.read().await.get_style().clone()
    }

    /// Set the style of the logger.
    pub async fn set_style(&mut self, style: fmt::Style) {
        self.formatter.write().await.set_style(style);
    }

    /// Stops the logger and waits for it to finish.
    ///
    /// This function takes ownership of the logger, and returns the result of the thread. (if the logger was stopped successfully)
    pub fn stop(self) -> thread::Result<()> {
        self.sender.close();
        self.handle.join()
    }

    /// Send a log to the logger.
    pub async fn send_log(&self, log: Log) {
        if let Err(e) = self.sender.send(log).await {
            eprintln!("Error sending log: {e:#?}, {}", e);
        }
    }

    pub async fn set_global_stream<T>(&mut self, stream: T)
        where
            T: Write + Send + Sync + 'static
    {
        self.register_stream(GLOBAL_STREAM_ID, stream).await;
    }

    /// Registers a stream with the logger.
    ///
    /// # Example
    /// ```rs
    /// let mut logger = Logger::new();
    /// let stream = std::io::stdout();
    ///
    /// logger.register_stream("test", stream).await;
    ///
    /// assert!(logger.is_stream_registered("test").await);
    /// ```
    pub async fn register_stream<'a, T, S>(&'a mut self, id: S, stream: T)
        where
            T: Write + Send + Sync + 'a + 'static,
            S: Into<StreamID>
    {
        self.streams
            .write().await
            .insert(
                id.into(),
                Box::new(stream)
            );
    }

    /// Removes a stream from the logger.
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut logger = Logger::new();
    /// let stream = std::io::stdout();
    ///
    /// logger.register_stream("test", stream).await;
    /// assert!(logger.is_stream_registered("test").await);
    ///
    /// logger.unregister_stream("test").await;
    /// assert!(!logger.is_stream_registered("test").await);
    /// ```
    pub async fn unregister_stream<S>(&mut self, id: S)
        where
            S: Into<StreamID>
    {
        self.streams
            .write().await
            .remove(&id.into());
    }

    /// Checks if a stream is registered with the logger.
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut logger = Logger::new();
    /// let stream = std::io::stdout();
    ///
    /// logger.register_stream("test", stream).await;
    ///
    /// assert!(logger.is_stream_registered("test").await);
    /// ```
    pub async fn is_stream_registered<S>(&self, id: S) -> bool
        where
            S: Into<StreamID>
    {
        self.streams
            .read().await
            .contains_key(&id.into())
    }
}

impl Deref for Logger {
    type Target = Arc<RwLock<HashMap<StreamID, Stream>>>;

    fn deref(&self) -> &Self::Target {
        &self.streams
    }
}

pub fn info(l: &Logger, log: impl Into<Log>) {
    smol::block_on(async {
        l.send_log(log.into()).await;
    });
}

pub fn panic(lg: &Logger, m: impl ToString) {
    let mut l = Log::new(m.to_string());
    l.set_log_type(log::LogType::Panic);

    smol::block_on(async {
        lg.send_log(l).await;
    });
}

pub fn error(lg: &Logger, m: impl ToString) {
    let mut l = Log::new(m.to_string());
    l.set_log_type(log::LogType::Error);

    smol::block_on(async {
        lg.send_log(l).await;
    });
}

pub fn warn(lg: &Logger, m: impl ToString) {
    let mut l = Log::new(m.to_string());
    l.set_log_type(log::LogType::Warn);

    smol::block_on(async {
        lg.send_log(l).await;
    });
}

pub fn debug(lg: &Logger, m: impl ToString) {
    let mut l = Log::new(m.to_string());
    l.set_log_type(log::LogType::Debug);

    smol::block_on(async {
        lg.send_log(l).await;
    });
}

pub fn trace(lg: &Logger, m: impl ToString) {
    let mut l = Log::new(m.to_string());
    l.set_log_type(log::LogType::Trace);

    smol::block_on(async {
        lg.send_log(l).await;
    });
}

// define macros for the log levels
#[macro_export]
macro_rules! info {
    ($l:ident, $m:expr) => {
        $crate::info(&$l, $m);
    };
    ($l:ident, $m:expr, $($arg:tt)*) => {
        $crate::info(&$l, format!($m, $($arg)*));
    };
}

#[macro_export]
macro_rules! panic {
    ($l:ident, $m:expr) => {
        $crate::panic(&$l, $m);
    };
    ($l:ident, $m:expr, $($arg:tt)*) => {
        $crate::panic(&$l, format!($m, $($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($l:ident, $m:expr) => {
        $crate::error(&$l, $m);
    };
    ($l:ident, $m:expr, $($arg:tt)*) => {
        $crate::error(&$l, format!($m, $($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($l:ident, $m:expr) => {
        $crate::warn(&$l, $m);
    };
    ($l:ident, $m:expr, $($arg:tt)*) => {
        $crate::warn(&$l, format!($m, $($arg)*));
    };
}

#[macro_export]
macro_rules! debug {
    ($l:ident, $m:expr) => {
        $crate::debug(&$l, $m);
    };
    ($l:ident, $m:expr, $($arg:tt)*) => {
        $crate::debug(&$l, format!($m, $($arg)*));
    };
}

#[macro_export]
macro_rules! trace {
    ($l:ident, $m:expr) => {
        $crate::trace(&$l, $m);
    };
    ($l:ident, $m:expr, $($arg:tt)*) => {
        $crate::trace(&$l, format!($m, $($arg)*));
    };
}

#[macro_export]
macro_rules! log {
    ($l:ident, $t:ident, $m:expr) => {
        $crate::$t(&$l, $m);
    };
    ($l:ident, $t:ident, $m:expr, $($arg:tt)*) => {
        $crate::$t(&$l, format!($m, $($arg)*));
    };
}


#[cfg(test)]
mod test {
    #[test]
    fn register_stream(){
        let mut logger = super::Logger::new();

        let stream = std::io::stdout();

        smol::block_on(async {
            logger.register_stream("test", stream).await;

            assert!(logger.is_stream_registered("test").await);
        });

    }

    #[test]
    #[cfg(target_os = "linux")]
    fn register_file_stream(){
        let mut logger = super::Logger::new();

        let file = std::fs::File::create("/tmp/test_aegis_server.log").unwrap();

        smol::block_on(async {
            logger.register_stream("test", file).await;

            assert!(logger.is_stream_registered("test").await);
        });
    }

    #[test]
    fn unregister_stream(){
        let mut logger = super::Logger::new();

        let stream = std::io::stdout();

        smol::block_on(async {
            logger.register_stream("test", stream).await;
            assert!(logger.is_stream_registered("test").await);

            logger.unregister_stream("test").await;
            assert!(!logger.is_stream_registered("test").await);
        });
    }
}