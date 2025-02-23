use http_server::{constants::Context, server};
use std::{collections::HashMap, net::TcpListener, thread};

struct Server {
    port: String,
    routes: HashMap<String, fn(&mut Context)>,
}

trait HttpServerTrait {
    fn new(port: &str) -> Self;
    fn run(&self, message: &str) -> std::io::Result<()>;
    fn get(&mut self, route: &str, handler: fn(&mut Context));
}

impl HttpServerTrait for Server {
    fn new(port: &str) -> Self {
        return Server {
            port: port.to_string(),
            routes: HashMap::new(),
        };
    }

    fn run(&self, message: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))?;

        println!("{} \r\n", message);

        for stream in listener.incoming() {
            let routes = self.routes.clone();
            match stream {
                Ok(mut socket) => {
                    thread::spawn(move || {
                        server::handler::handle_client(&mut socket, &routes);
                    });
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }

        Ok(())
    }

    fn get(&mut self, route: &str, handler: fn(&mut Context)) {
        self.routes.insert(route.to_string(), handler);
    }
}

fn main() -> std::io::Result<()> {
    let mut server = Server::new("4000");

    server.get("/home", |ctx| {
        ctx.response_text("home page");
    });

    if let Err(e) = server.run("Server is running on port 4000") {
        eprintln!("failed to start server: {}", e);
        std::process::exit(1)
    }

    Ok(())
}
