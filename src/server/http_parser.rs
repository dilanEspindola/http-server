use std::{borrow::Cow, collections::HashMap};

struct Request {
    mathod: String,
    path: String,
    http_version: String,
    header: HashMap<String, String>,
    body: Option<String>,
}

pub fn parser(request: &Cow<'_, str>) -> Request {
    let split_request: Vec<&str> = request.split(' ').collect();

    println!("{}", request);

    // for request_item in &split_request[1..] {
    //     println!("request item {}", request_item);
    // }

    for request_item in split_request {
        println!("request item {}", request_item);
    }

    // mut return request, path,  http version, body, headers

    return {
        Request {
            mathod: "".to_string(),
            path: "".to_string(),
            http_version: "".to_string(),
            header: HashMap::new(),
            body: Some("".to_string()),
        }
    };
}
