//! TODO: Add documentation
//! TODO: Add functions to send specific log levels
//! TODO: Add macros to send logs (e.g. info!(l: logger, "Hello, world!"))
//! TODO: Open the Style api
//! TODO: Open the rules api
//! TODO: Test everything above (shit)
pub mod rules;
pub mod log;
mod runner;
pub mod fmt;


use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
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
    formatter: Arc<fmt::Fmt>
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
        let fmt = Arc::new(fmt::Fmt::default());

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

    pub fn stop(self) -> thread::Result<()> {
        self.sender.close();
        self.handle.join()
    }

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