use crate::{constants::insert_reuquest_method_before, constants::Context, server::http_parser};
use std::collections::HashMap;

/* process_request function processes uncoming request and returns the response and response may be text/plain or json. Also responds a 4040 if route not found */

/**
* the handler function is the funciton that was declared in the main.rs file
* the handler is the |ctx| {...code }
*     server.get("/home", |ctx| {
         ctx.text_plain("this is home route");
       });
* in the previous example we are calling the text_plain() function
* the text_plain() function is the responsible for saving the value in the context(response_text)
* when the value is saved in the context, you can access to those properties, that's why we are passing a mutable reference in the handler() below.
*
* We are saving the method, path because we may use them in the main.rs file
*/
pub fn process_request(
    routes: &HashMap<String, fn(&mut Context)>,
    request: http_parser::Request,
) -> String {
    let path = insert_reuquest_method_before(&request.path, &request.method);

    if let Some(handler) = routes.get(&path) {
        let mut context = Context::new();
        context.save_method(&request.method);
        context.save_path(&request.path);

        handler(&mut context);

        if let Some(text_response) = context.response_text {
            return format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                text_response.len(),
                text_response
            );
        }

        if let Some(json_response) = context.json_response {
            return format!("HTTP/1.1 200 OK \r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", json_response.len(), json_response);
        }
    }

    // returns if route not found
    return format!(
        "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: 9\r\n\r\nNot Found"
    );
}
