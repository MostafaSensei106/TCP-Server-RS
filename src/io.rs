use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let row_bytes_read = stream.read(&mut buffer).expect(&format!(
        "[{}] TCP Stream Error: Failed to read form client!",
        crate::time::current_timestamp()
    ));
    let request = String::from_utf8_lossy(&buffer[..row_bytes_read]);
    let peer = stream
        .peer_addr()
        .map_or_else(|_| "Unknon Peer".to_string(), |addr| addr.to_string());
    println!(
        "[{}] Received request: {} bytes from {}:{}",
        crate::time::current_timestamp(),
        row_bytes_read,
        peer,
        request
    );

    let response = "Hello, My Love Client!".as_bytes();

    stream.write(response).expect(&format!(
        "[{}] TCP Streem Error: Failed to wite responeto Clinet",
        crate::time::current_timestamp(),
    ));
}
