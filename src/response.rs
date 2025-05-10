use alloc::string::String;
use alloc::vec::Vec;

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

        let mut lines = preprocessed_responses.lines();
        let status_line = lines.next().unwrap_or("");
        let mut parts = status_line.split_whitespace();

        let version = parts.next().unwrap_or("").to_string();
        let status_code: u16 = parts.next().unwrap_or("0").parse().unwrap_or(0);
        let reason_phrase = parts.collect::<Vec<&str>>().join(" ");

        let header_strings: Vec<String> = lines.map(|line| line.to_string()).collect();
        let body = header_strings.last().unwrap_or(&"".to_string()).to_string();

        let headers: Vec<Header> = header_strings
            .iter()
            .filter_map(|line| {
                if line.contains(": ") {
                    Some(Header::new(line.clone()))
                } else {
                    None
                }
            })
            .collect();

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