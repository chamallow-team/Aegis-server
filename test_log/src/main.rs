use std::fs::File;
use std::io::Read;
use logs::fmt::{DateStyle, Style};
use logs::Logger;
use logs::rules::{Rule, RuleType};
use regex::Regex;
use logs::log::LogType;

fn main() {
    let mut logger = Logger::new();

    let mut style = Style::default();
    style.set_pattern("[{date}] \x1b[37m{{type}}\x1b[0m {route} {c}{text}{sc}");
    style.set_date_style(DateStyle::HourMinuteSecond);

    // define the new style
    smol::block_on(async { logger.set_style(style).await; });

    // define streams & routes
    smol::block_on(async {

        // global

        logger.set_global_stream(std::io::stdout()).await;

        logger.add_global_rule(
            Rule::new(RuleType::ExcludeRoute)
                .set_pattern(Regex::new("api::backend").unwrap())
        ).await;

        // Greater than debug

        let f = File::create("/tmp/test_all_aegis_server.log").expect("Failed to create file");
        f.set_len(0).expect("Failed to clear file");
        logger.register_stream("logs_all", f).await;

        logger.add_route_rule(
            "logs_all",
            Rule::new(RuleType::GreaterLevelThan)
                .set_lvl(LogType::Debug)
        ).await;

        // traces & debugs

        let f = File::create("/tmp/test_trace_aegis_server.log").expect("Failed to create file");
        f.set_len(0).expect("Failed to clear file");
        logger.register_stream("logs_trace", f).await;

        logger.add_route_rule(
            "logs_trace",
            Rule::new(RuleType::RequireRoute)
                .set_pattern(Regex::new("api::backend").unwrap())
        ).await;
    });

    // test
    smol::block_on(async {
        logger.send_log("Test 1".into()).await;
        logger.send_log("Test 2".into()).await;

        logs::panic!(logger, "Panic!");
        logs::trace!(logger, "the program panicked!");

        logs::debug!(logger, route: "api::backend", "Debug on the backend");
    });

    logger.stop().expect("Failed to stop logger");

    let mut f = File::open("/tmp/test_aegis_server.log").expect("Failed to open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Failed to read file");

    println!("\n\n/tmp/test_aegis_server.log contents:\n\n{}", contents);

    let mut f = File::open("/tmp/test_trace_aegis_server.log").expect("Failed to open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Failed to read file");

    println!("\n\n/tmp/test_trace_aegis_server.log contents:\n\n{}", contents);
}
