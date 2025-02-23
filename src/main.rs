use http_server::server;
use std::{net::TcpListener, thread};

struct Server {
    port: String,
}

trait HttpServerTrait {
    fn new(port: &str) -> Self;
    fn run(self, message: &str) -> std::io::Result<()>;
}

impl HttpServerTrait for Server {
    fn new(port: &str) -> Self {
        return Server {
            port: port.to_string(),
        };
    }

    fn run(self, message: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))?;

        println!("{} \r\n", message);

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
}

fn main() -> std::io::Result<()> {
    let server = Server::new("4000");

    if let Err(e) = server.run("Server running on port 4000") {
        eprintln!("failed to start server: {}", e);
        std::process::exit(1)
    }

    Ok(())
}
