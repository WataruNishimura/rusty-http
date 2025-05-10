use std::os::macos::raw::stat;

use alloc::string::String;
use alloc::vec::Vec;
use trust_dns_resolver::proto::op::header;

use crate::header::Header;

pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<Header>,
    pub body: String,
    pub version: String,
    pub reason_phrase: String,
}

impl HttpResponse {
    pub fn new(raw_response: String) -> Self {

        let preprocessed_responses = raw_response.trim_start()
            .replace("\r\n\r\n", "\r\n")
            .replace("\r\n", "\n");

        let (status_line, headers_body) = match preprocessed_responses.split_once("\n") {
            Some((status_line, headers_body)) => (status_line, headers_body),
            None => panic!("Invalid HTTP response format"),
        };

        let (headers, body) = match headers_body.split_once("\n\n") {
            Some((h, b)) => {
              let mut headers = Vec::new();
              for header in h.lines() {
                headers.push(Header::new(header.to_string()));
              }

              (headers, b.to_string())
            },
            None => panic!("Invalid HTTP response format"),
        };  

        let status_parts: Vec<&str> = status_line.split_whitespace().collect();
        if status_parts.len() < 3 {
            panic!("Invalid HTTP response format");
        }
        let version = status_parts[0].to_string();
        let status_code: u16 = status_parts[1].parse().unwrap_or(0);
        let reason_phrase = status_parts[2..].join(" ");

        Self {
            status_code,
            headers,
            body,
            version,
            reason_phrase,
        }
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn headers(&self) -> &[Header] {
        &self.headers
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn reason_phrase(&self) -> &str {
        &self.reason_phrase
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_one_header() {
        let raw_response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\nHello, world!";
        let response = HttpResponse::new(raw_response.to_string());

        assert_eq!(response.status_code(), 200);
        assert_eq!(response.reason_phrase(), "OK");
        assert_eq!(response.body(), "Hello, world!");
        assert_eq!(response.version(), "HTTP/1.1");
        assert_eq!(response.headers().len(), 1);
    }
    #[test]
    fn test_multiple_headers() {
        let raw_response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 13\r\n\r\nHello, world!";
        let response = HttpResponse::new(raw_response.to_string());

        assert_eq!(response.status_code(), 200);
        assert_eq!(response.reason_phrase(), "OK");
        assert_eq!(response.body(), "Hello, world!");
        assert_eq!(response.version(), "HTTP/1.1");
        assert_eq!(response.headers().len(), 2);
    }
}