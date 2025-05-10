extern crate alloc; 
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};

use alloc::string::String;
use alloc::vec::Vec;

use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

use crate::http::alloc::string::ToString;

use crate::response::HttpResponse;
use crate::error::Error;

pub struct HttpClient {

}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get (&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
      let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

      let ips = match resolver.lookup_ip(host) {
          Ok(response) => response.iter().collect::<Vec<IpAddr>>(),
          Err(e) => {
            return Err(Error::Network("Failed to find IP addresses".to_string()));
          }
      };

      if ips.len() < 1 {
        return Err(Error::Network("No IP addresses found".to_string()));
      }

      
      let socket_addr = SocketAddr::new(ips[0], port);

      // TCPコネクションを構築
      let mut stream = match TcpStream::connect(socket_addr) {
          Ok(mut stream) => stream,
          Err(e) => {
            return Err(Error::Network("Failed to connect to server".to_string()));
          }
      };

      // HTTPリクエストを作成
      let request_line: String = format!("GET {} HTTP/1.1\r\nHost: {}\r\n", path, host);

      let headers: Vec<String> = vec![
          format!("Host: {}", host),
          "Connection: close".to_string(),
          "User-Agent: RustyHttp/0.1".to_string(),
          "Accept: */*".to_string(),
      ];  

      let request: String = format!("{}\r\n{}\r\n", request_line, headers.join("\r\n"));

      let bytes_written = match stream.write(request.as_bytes()) {
          Ok(bytes) => bytes,
          Err(e) => {
            return Err(Error::Network("Failed to write to server".to_string()));
          }
      };

      let mut received = Vec::new();

      loop {
          let mut buffer = [0u8; 4096];
          let bytes_read= match stream.read(&mut buffer) {
              Ok(n) => n,
              Err(_e) => {
                return Err(Error::Network("Failed to read from server".to_string()));
              }
          };

          if bytes_read == 0 {
              break;
          }

          received.extend_from_slice(&buffer[..bytes_read]);
      }

      match core::str::from_utf8(&received) {
          Ok(response_str) => {
              let response = HttpResponse::new(response_str.to_string());
              Ok(response)
          }
          Err(_e) => {
              Err(Error::Network("Failed to parse response".to_string()))
          }
      }
      
    }
}