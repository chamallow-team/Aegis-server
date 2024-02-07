use std::fs::File;
use std::io::Read;
use logs::log::LogType;
use logs::Logger;

fn main() {
    let mut logger = Logger::new();

    smol::block_on(async {
        logger.set_global_stream(std::io::stdout()).await;

        let f = File::create("/tmp/test_aegis_server.log").expect("Failed to create file");

        logger.register_stream("test", f).await;

        logger.send_log("Test 1".into()).await;
        logger.send_log("Test 2".into()).await;

        logger.send_log((LogType::Error, "Test 3").into()).await;
    });

    logger.stop().expect("Failed to stop logger");

    let mut f = File::open("/tmp/test_aegis_server.log").expect("Failed to open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Failed to read file");

    println!("\n\n/tmp/test_aegis_server.log contents:\n\n{}", contents);
}
