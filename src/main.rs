use http_server::server::server_handler::{self, HttpServerTrait};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let mut server = server_handler::Server::new("4000");

    server.get("/home", |ctx| {
        ctx.text_plain("GET - home rotue");
    });

    server.get("/post", |ctx| {
        let mut response = HashMap::new();
        response.insert("message", "GET - post route");
        ctx.json(response);
    });

    server.post("/post", |ctx| {
        let mut response = HashMap::new();
        response.insert("message", "POST_CREATED");
        ctx.json(response);
    });

    server.get("/hello", |ctx| {
        ctx.text_plain("GET - hello route");
    });

    server.get("/contact", |ctx| {
        let mut map_response = HashMap::new();
        map_response.insert("status".to_string(), "200".to_string());
        map_response.insert("message".to_string(), "contact page".to_string());

        ctx.json(map_response);
    });

    if let Err(e) = server.run("Server is running on port 4000") {
        eprintln!("failed to start server: {}", e);
        std::process::exit(1)
    }

    Ok(())
}
