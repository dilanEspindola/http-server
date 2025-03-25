#[cfg(test)]
mod tests {
    use crate::server::server_handler::{self, HttpServerTrait};
    use serde_json::json;
    use std::net::TcpListener;

    #[test]
    fn run_server_with_routes() {
        let mut server = server_handler::Server::new("3000");

        server.get("/home", |ctx| {
            ctx.text_plain("GET - home rotue");
        });

        server.post("/post", |ctx| {
            let body = ctx.body.as_mut().unwrap().clone();

            ctx.json(json!({
                "message": "POST_CREATED",
                "content": body
            }));
        });

        server.put("/put-route", |ctx| {
            ctx.json(json!({
                "status": 200,
                "message": "PUT - put route"
            }));
        });

        server.patch("/patch-route", |ctx| {
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

        let result = server.run("Server is running on port 3000");

        assert!(result.is_ok());
    }

    #[test]
    fn run_server_with_error() {
        // This test will fail because the blockiing_port is blocking the port 2000
        let blockiing_port =
            TcpListener::bind("127.0.0.1:2000").expect("Server blocking port 2000");

        let server = server_handler::Server::new("2000");
        let result = server.run("Server is running on port 2000");

        assert!(result.is_err());
    }
}
