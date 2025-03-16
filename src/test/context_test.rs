#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::server::context::{self, Context};
    use crate::server::http_parser;

    #[test]
    fn create_context() {
        let context = Context::new();

        assert!(context.method.is_none());
        assert_eq!(context.path, None);
        assert!(context.headers.is_none());
        assert!(context.body.is_none());
        assert!(context.query.is_none());
        assert!(context.params.is_none());
        assert!(context.response_text.is_none());
        assert!(context.json_response.is_none());
    }

    #[test]
    fn save_method() {
        let mut context = Context::new();
        context.method = Some("GET".to_string());

        assert_eq!(context.method, Some("GET".to_string()));
    }

    #[test]
    fn save_path() {
        let mut context = Context::new();
        context.path = Some("/home".to_string());

        assert_eq!(context.path, Some("/home".to_string()));
    }

    #[test]
    fn text_plain() {
        let mut context = Context::new();
        context.text_plain("hello");

        assert_eq!(context.response_text, Some("hello".to_string()));
    }

    #[test]
    fn json() {
        let mut context = Context::new();
        let response = json!({"message": "hello"});
        context.json(response);

        assert_eq!(
            context.json_response,
            Some(r#"{"message":"hello"}"#.to_string())
        );
    }

    #[test]
    fn save_body() {
        let mut context = Context::new();

        let request_line =
            String::from_utf8_lossy(b"POST /post HTTP/1.1\r\nHost: localhost:4000\r\nContent-Type: application/json\r\nAccept: application/json, text/plain, */*\r\nAccept-Encoding: gzip, deflate, br\r\nConnection: keep-alive\r\nContent-Length: 167\r\nAuthorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IlRlc3QgVXNlciJ9.vq1u0DcNGB9r3U4dM_1oGveIQA6Yz8XkJOzHDIqiZPQ\r\n\r\n{\"user\":{\"id\":12345,\"name\":\"John Doe\",\"email\":\"john.doe@example.com\"},\"action\":\"update_profile\",\"timestamp\":1678901234}");

        let request = http_parser::parser(&request_line);
        let body = request.body.clone().unwrap();

        context.save_body(body);

        assert_eq!(context.body.unwrap(), request.body.unwrap());
    }

    #[test]
    fn save_headers() {
        let mut context = Context::new();

        let request_line =
            String::from_utf8_lossy(b"POST /post HTTP/1.1\r\nHost: localhost:4000\r\nContent-Type: application/json\r\nAccept: application/json, text/plain, */*\r\nAccept-Encoding: gzip, deflate, br\r\nConnection: keep-alive\r\nContent-Length: 167\r\nAuthorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IlRlc3QgVXNlciJ9.vq1u0DcNGB9r3U4dM_1oGveIQA6Yz8XkJOzHDIqiZPQ\r\n\r\n{\"user\":{\"id\":12345,\"name\":\"John Doe\",\"email\":\"john.doe@example.com\"},\"action\":\"update_profile\",\"timestamp\":1678901234}");

        let request = http_parser::parser(&request_line);
        let headers = request.headers.clone();

        context.save_headers(headers);

        assert_eq!(context.headers.unwrap(), request.headers);
    }

    #[test]
    fn insert_request_method_before() {
        let result =
            context::insert_request_method_before(&String::from("/home"), &String::from("GET"));

        assert_eq!(result, "GET-/home");
    }
}
