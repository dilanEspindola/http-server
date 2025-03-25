use http_server::server::server_handler::{self, HttpServerTrait};
use serde_json::json;

fn main() -> std::io::Result<()> {
    let mut server = server_handler::Server::new("4000");

    server.get("/post", |ctx| {
        ctx.text_plain("GET - post path");
    });

    server.post("/post", |ctx| {
        let body = ctx.body.as_mut().unwrap();
        let response = json!({
            "message": "POST_CREATED",
            "content": body
        });
        ctx.json(response);
    });

    server.get("/contact", |ctx| {
        ctx.json(json!({
            "status": 200,
            "message": "GET - contact route"
        }));
    });

    server.put("/put-route", |ctx| {
        ctx.json(json!({
            "status": 200,
            "message": "PUT - put route"
        }));
    });

    server.patch("/patch-route/param:id", |ctx| {
        ctx.json(json!({
           "status": 200,
            "message": "PATCH - patch route"
        }));
    });

    server.delete("/delete-route", |ctx| {
        ctx.json(json!({
            "status": 200,
            "message": "DELETE - delete route"
        }));
    });

    if let Err(e) = server.run("Server is running on port 4000") {
        eprintln!("failed to start server: {}", e);
        std::process::exit(1)
    }

    Ok(())
}
