use crate::{
    constants::Context,
    server::{http_parser, process_request},
};
use std::{
    collections::HashMap,
    io::{Read, Write},
};

pub fn process_client_communication(
    socket: &mut std::net::TcpStream,
    routes: &HashMap<String, fn(&mut Context)>,
) {
    let mut buffer = [0; 1024];

    match socket.read(&mut buffer) {
        Ok(n) => {
            let http_line_request = String::from_utf8_lossy(&buffer[..n]);
            let request = http_parser::parser(&http_line_request);

            if http_line_request.contains("GET /favicon.ico HTTP/1.1") {
                let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
                socket.write_all(response.as_bytes()).unwrap();
                return;
            }

            let response = process_request::process_request(routes, request);

            match socket.write_all(response.as_bytes()) {
                Ok(_n) => {
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
