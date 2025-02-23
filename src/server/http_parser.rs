use std::{borrow::Cow, collections::HashMap};

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub http_version: String,
    pub header: HashMap<String, String>,
    pub body: Option<String>,
}

pub fn parser(request: &Cow<'_, str>) -> Request {
    let split_request: Vec<&str> = request.split(' ').collect();
    let main_request_http = &split_request.clone()[0..3];

    let mut headers = HashMap::new();

    for header in request.split("\r\n").skip(1) {
        let split_header: Vec<&str> = header.split(':').map(|s| s.trim()).collect();

        for (key, value) in split_header.iter().zip(split_header.iter().skip(1)) {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    return Request {
        method: main_request_http[0].to_string(),
        path: main_request_http[1].to_string(),
        http_version: main_request_http[2].to_string(),
        header: headers,
        body: None,
    };
}
