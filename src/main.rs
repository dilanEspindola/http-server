use http_server::server::server_handler::{self, HttpServerTrait};
use serde_json::json;

fn main() -> std::io::Result<()> {
    let mut server = server_handler::Server::new("4000");

    server.get("/home", |ctx| {
        ctx.text_plain("GET - home rotue");
    });

    server.get("/post", |ctx| {
        ctx.text_plain("GET - post path");
    });

    server.post("/post", |ctx| {
        let body = ctx.body.as_mut().unwrap();
        let product = body.get("product").unwrap();
        let user = body.get("user_data").unwrap();
        let response = json!({
            "message": "POST_CREATED",
            "product": product,
            "user": user
        });
        ctx.json(response);
    });

    server.get("/hello", |ctx| {
        ctx.text_plain("GET - hello route");
    });

    server.get("/contact", |ctx| {
        ctx.json(json!({
            "status": 200,
            "message": "GET - contact route"
        }));
    });

    if let Err(e) = server.run("Server is running on port 4000") {
        eprintln!("failed to start server: {}", e);
        std::process::exit(1)
    }

    Ok(())
}
