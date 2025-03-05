use serde_json::Value;
use std::{borrow::Cow, collections::HashMap};

pub struct Request {
    pub method: String,
    pub path: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<HashMap<String, Value>>,
}

// The parser is only able to parse JSON
// TODO: Add support for other formats
pub fn parser(request: &Cow<'_, str>) -> Request {
    let split_request: Vec<&str> = request.split("\r\n\r\n").collect();
    let first_part = split_request[0];
    let body = split_request[1];

    let main_request: Vec<&str> = first_part.split("\r\n").collect();
    let main_request_splitted: Vec<&str> = main_request.get(0).unwrap().split(" ").collect();

    let mut headers = HashMap::new();
    for header in main_request.iter().skip(1) {
        let removed_dots: Vec<&str> = header.split(": ").collect();
        let key = removed_dots[0];
        let value = removed_dots[1];
        headers.insert(key.to_string(), value.to_string());
    }

    if body.is_empty() {
        return Request {
            method: main_request_splitted[0].to_string(),
            path: main_request_splitted[1].to_string(),
            http_version: main_request_splitted[2].to_string(),
            headers: headers,
            body: None,
        };
    }

    let body_parsed = match serde_json::from_str::<serde_json::Value>(&body.to_string()) {
        Ok(s) => {
            let hashmap: HashMap<String, Value> =
                serde_json::from_str(s.to_string().as_str()).unwrap();
            hashmap
        }
        Err(_) => {
            panic!("Error parsing body");
        }
    };

    return Request {
        method: main_request_splitted[0].to_string(),
        path: main_request_splitted[1].to_string(),
        http_version: main_request_splitted[2].to_string(),
        headers: headers,
        body: Some(body_parsed),
    };
}
