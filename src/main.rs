use std::net::TcpListener;

use tcp_server_rs::{constants::Constants, io::handle_client};

fn main() {
    let network_listener = TcpListener::bind(format!(
        "{}:{}",
        Constants::default().addr,
        Constants::default().port
    ))
    .expect(&format!(
        "[{}] Network Listener Error: Failed to bind to address {}:{}",
        tcp_server_rs::time::current_timestamp(),
        Constants::default().addr,
        Constants::default().port
    ));
    println!(
        "[{}] Server listening to {}:{}",
        tcp_server_rs::time::current_timestamp(),
        Constants::default().addr,
        Constants::default().port
    );

    for stream in network_listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!(
                "[{}] Faild to establish connection: {}",
                tcp_server_rs::time::current_timestamp(),
                e
            ),
        }
    }
}
