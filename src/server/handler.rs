use crate::server::http_parser;
use std::io::{Read, Write};

pub fn handle_client(socket: &mut std::net::TcpStream) {
    let mut buffer = [0; 1024];

    match socket.read(&mut buffer) {
        Ok(n) => {
            let http_line_request = String::from_utf8_lossy(&buffer[..n]);
            http_parser::parser(&http_line_request);
            let response ="HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, World!";

            match socket.write(response.as_bytes()) {
                Ok(_n) => {
                    println!("Response sent",);
                    socket.flush().unwrap();
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
