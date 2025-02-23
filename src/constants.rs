use std::collections::HashMap;

pub enum HttpMethods {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}

#[derive(Debug, Clone)]
pub struct Context {
    method: Option<String>,
    path: Option<String>,
    query: Option<HashMap<String, String>>,
    params: Option<HashMap<String, String>>,
    pub response_text: Option<String>,
}

impl Context {
    pub fn new() -> Self {
        return Context {
            method: None,
            path: None,
            query: None,
            params: None,
            response_text: None,
        };
    }

    pub fn method(&mut self, method: &str) {
        self.method = Some(method.to_string());
    }

    pub fn path(&mut self, path: &str) {
        self.path = Some(path.to_string());
    }

    pub fn response_text(&mut self, text_plain: &str) {
        self.response_text = Some(text_plain.to_string());
    }
}
