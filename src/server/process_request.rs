use crate::{constants::Context, server::http_parser};
use std::collections::HashMap;

/**
 * THIS FUNCTION HANDLES THE ROUTES:
 * - Not found route
 * - if route exists it returns the response formated
 * - it returns text/plain
 * - it return json
 */
pub fn process_request(
    routes: &HashMap<String, fn(&mut Context)>,
    request: http_parser::Request,
) -> String {
    if let Some(handler) = routes.get(&request.path) {
        let mut context = Context::new();
        context.method(&request.method);
        context.path(&request.path);

        handler(&mut context);

        if let Some(text_response) = &context.response_text {
            return format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                text_response.len(),
                text_response
            );
        }

        if let Some(json_response) = &context.json_response {
            return format!("HTTP/1.1 200 OK \r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", json_response.len(), json_response);
        }
    }

    // returns if route not found
    return format!(
        "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: 9\r\n\r\nNot Found"
    );
}
