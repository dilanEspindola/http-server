use std::collections::HashMap;

use serde::Serialize;

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
    pub method: Option<String>,
    pub path: Option<String>,
    pub query: Option<HashMap<String, String>>,
    pub params: Option<HashMap<String, String>>,
    pub response_text: Option<String>,
    pub json_response: Option<String>,
}

impl Context {
    pub fn new() -> Self {
        return Context {
            method: None,
            path: None,
            query: None,
            params: None,
            response_text: None,
            json_response: None,
        };
    }

    pub fn save_method(&mut self, method: &str) {
        self.method = Some(method.to_string());
    }

    pub fn save_path(&mut self, path: &str) {
        self.path = Some(path.to_string());
    }

    pub fn text_plain(&mut self, text_plain: &str) {
        self.response_text = Some(text_plain.to_string());
    }

    pub fn json(&mut self, map_response: impl Serialize) {
        let serialized = serde_json::to_string(&map_response).unwrap();
        self.json_response = Some(serialized);
    }
}
