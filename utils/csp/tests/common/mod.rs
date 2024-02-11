use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

// ======================= Server =======================
pub fn listen(addr: &str) -> (thread::JoinHandle<()>, Arc<Mutex<bool>>) {
    let mutex = Arc::new(Mutex::new(false));
    let mutex2 = Arc::clone(&mutex);

    let addr = addr.to_string();
    (
        thread::spawn(move || {
            let mut guard = mutex2.lock().unwrap();
            let server = TcpListener::bind(addr).unwrap();
            *guard = true;
            drop(guard);
            println!("listen");

            for client in server.incoming() {
                if client.is_ok() {
                    handle_client(client.unwrap())
                }
            }
        }),
        mutex,
    )
}

fn handle_client(client: TcpStream) {
    // TODO: handle csp parsing and responding
}
