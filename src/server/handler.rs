use crate::server::http_parser::{self, Request};
use std::io::{Read, Write};

pub fn handle_client(socket: &mut std::net::TcpStream) {
    let mut buffer = [0; 1024];

    match socket.read(&mut buffer) {
        Ok(n) => {
            let http_line_request = String::from_utf8_lossy(&buffer[..n]);
            let request = http_parser::parser(&http_line_request);
            println!("request {}", request.http_version);
            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, World!";

            if http_line_request.contains("GET /favicon.ico HTTP/1.1") {
                let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
                socket.write_all(response.as_bytes()).unwrap();
                return;
            }

            match socket.write_all(response.as_bytes()) {
                Ok(_n) => {
                    println!("Response sent\r\n");
                    socket.flush().unwrap();
                    socket.shutdown(std::net::Shutdown::Both).unwrap();
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
