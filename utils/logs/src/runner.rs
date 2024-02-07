use std::collections::HashMap;
use std::sync::Arc;
use smol::lock::RwLock;
use crate::{GLOBAL_STREAM_ID, Stream, StreamID};
use crate::fmt::Fmt;
use crate::log::Log;
use crate::rules::Rules;

pub(crate) async fn run(
    streams_arc: Arc<RwLock<HashMap<StreamID, Stream>>>,
    rules: Arc<RwLock<Rules>>,
    fmt: Arc<Fmt>,
    log: Log
) {
    let rules = rules.read().await;

    let streams = streams_arc.read().await;
    let keys = streams.keys().map(|s| s.to_string()).collect::<Vec<String>>();
    drop(streams);

    let mut formatted = log.text().to_string();
    let mut is_formatted = false;

    if keys.contains(&GLOBAL_STREAM_ID.into()) && rules.is_global_allowed(&log) {
        let mut streams = streams_arc.write().await;
        // write to global stream
        if let Some(stream) = streams.get_mut(GLOBAL_STREAM_ID) {
            formatted = fmt.format(&log);
            is_formatted = true;

            // write to the stream with \n at the end
            if let Err(e) = stream.write(formatted.as_bytes()).map(|_| stream.write(b"\n")) {
                eprintln!("Error writing to global stream: {e:#?}");
            }
        }
    }

    // for each stream, check if the log is allowed and write to the stream
    for id in keys {
        if id == GLOBAL_STREAM_ID {
            continue;
        }

        // if there is no rules, the first condition will not be met
        // else, the second condition will go to the next iteration if the log is not allowed by the rules
        if let Some(route_rules) = rules.get_route_rules_from_id(&id) {
            if !route_rules.is_allowed(&log) {
                continue;
            }
        }

        // if the conditions above aren't met, write to the stream

        if !is_formatted {
            formatted = fmt.format(&log);
            is_formatted = true;
        }

        let mut streams = streams_arc.write().await;
        if let Some(stream) = streams.get_mut(&id) {
            // write to the stream with \n at the end
            if let Err(e) = stream.write(formatted.as_bytes()).map(|_| stream.write(b"\n")) {
                eprintln!("Error writing to stream {id}: {e:#?}");
            }
        }
    }

}