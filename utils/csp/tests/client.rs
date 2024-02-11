mod common;

#[test]
fn test_client() {
    let (thread, lock) = common::listen("127.0.0.1:8080");

    loop {
        // avoid connecting before the server can accept clients
        let guard = lock.lock().unwrap();
        if *guard {
            break;
        }
        // drop guard for us
    }

    println!("connecting");

    // TODO: send packets and handle responses

    thread.join().unwrap_or_default();
}
