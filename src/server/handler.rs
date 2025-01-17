use std::io::{Read, Write};

pub fn handle_client(socket: &mut std::net::TcpStream) {
    let mut buffer = [0; 1024];

    match socket.read(&mut buffer) {
        Ok(n) => {
            let http_line = String::from_utf8_lossy(&buffer[..n]);
            println!("Received {}", http_line);
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, World!");
            match socket.write(response.as_bytes()) {
                Ok(n) => {
                    println!("Response sent {}", n);
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
