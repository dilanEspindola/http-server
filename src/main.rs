use http_server::server;
use std::{net::TcpListener, thread};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;

    println!("Server running on {} \r\n", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(mut socket) => {
                thread::spawn(move || {
                    server::handler::handle_client(&mut socket);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}
