use crate::server::http_parser;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn http_parser() {
        let http_line_request =
            String::from_utf8_lossy(b"GET /home HTTP/1.1\r\nHost: localhost:4000\r\n\r\n");

        let request = http_parser::parser(&http_line_request);

        assert_eq!(request.method, "GET");
        assert_eq!(request.path, "/home");
        assert_eq!(request.http_version, "HTTP/1.1");
        assert_eq!(request.headers.get("Host").unwrap(), "localhost:4000");
    }

    #[test]
    fn http_parser_with_body() {
        let http_line_request =
            String::from_utf8_lossy(b"POST /post HTTP/1.1\r\nHost: localhost:4000\r\nContent-Type: application/json\r\nAccept: application/json, text/plain, */*\r\nAccept-Encoding: gzip, deflate, br\r\nConnection: keep-alive\r\nContent-Length: 167\r\nAuthorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IlRlc3QgVXNlciJ9.vq1u0DcNGB9r3U4dM_1oGveIQA6Yz8XkJOzHDIqiZPQ\r\n\r\n{\"user\":{\"id\":12345,\"name\":\"John Doe\",\"email\":\"john.doe@example.com\"},\"action\":\"update_profile\",\"timestamp\":1678901234}");

        let request = http_parser::parser(&http_line_request);

        assert_eq!(request.method, "POST");
        assert_eq!(request.path, "/post");
        assert_eq!(
            request.headers.get("Content-Type").unwrap(),
            "application/json"
        );
        assert_eq!(
            request.headers.get("Authorization").unwrap(),
            "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IlRlc3QgVXNlciJ9.vq1u0DcNGB9r3U4dM_1oGveIQA6Yz8XkJOzHDIqiZPQ"
        );
        assert_eq!(
            request.body.unwrap().get("user").unwrap().to_string(),
            "{\"email\":\"john.doe@example.com\",\"id\":12345,\"name\":\"John Doe\"}"
        );
    }
}
