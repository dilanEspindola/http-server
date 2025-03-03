use crate::constants::Context;
use crate::server;
use std::{collections::HashMap, net::TcpListener, thread};

pub struct Server {
    port: String,
    routes: HashMap<String, fn(&mut Context)>,
}

pub trait HttpServerTrait {
    fn new(port: &str) -> Self;
    fn run(&self, message: &str) -> std::io::Result<()>;
    fn get(&mut self, route: &str, handler: fn(&mut Context));
    fn post(&mut self, route: &str, handler: fn(&mut Context));
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
                        server::client_communication::process_client_communication(
                            &mut socket,
                            &routes,
                        );
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
        let path_method = format!("GET-{}", route);
        self.routes.insert(path_method, handler);
    }

    fn post(&mut self, route: &str, handler: fn(&mut Context)) {
        let path_method = format!("POST-{}", route);
        self.routes.insert(path_method, handler);
    }
}
