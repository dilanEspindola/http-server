use crate::{constants::Context, server::http_parser};
use std::{
    collections::HashMap,    
    io::{Read, Write},
};

pub fn handle_client(socket: &mut std::net::TcpStream, routes: &HashMap<String, fn(&mut Context)>) {
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

            let response = handle_routes(routes, request);

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

/**
 * THIS FUNCTION HANDLES THE ROUTES:
 * - Not foond route
 * - if route exists it returns the response formated
 * - it returns text/plain
 * TODO: handle json response because it's returning a text/plain by default
 */

fn handle_routes(
    routes: &HashMap<String, fn(&mut Context)>,
    request: http_parser::Request,
) -> String {
    let mut response = String::new();
    for (key, value) in routes.iter() {

        // returns the response of the route if exists
        if request.path == *key {
            let mut context = Context::new();
            context.method(&request.method);
            context.path(&request.path);

            value(&mut context);

            let response_text = context
                .response_text
                .as_ref()
                .map_or("default", String::as_str);

            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                response_text.len(),
                response_text
            );
            break;
        }

        // handles not found
        if request.path != *key {            
            let message_content = "page not found";
            response = format!("HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n Content-Length: {}\r\n\r\n{}", 
            message_content.len(), 
            message_content                        
            );
        }
    }

    return response;
}
