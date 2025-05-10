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

      let ips = match resolver.lookup_ip(&host) {
          Ok(response) => response.iter().collect::<Vec<IpAddr>>(),
          Err(_e) => {
            return Err(Error::Network("Failed to find IP addresses".to_string()));
          }
      };

      if ips.len() < 1 {
        return Err(Error::Network("No IP addresses found".to_string()));
      }

      println!("IP addresses for {}: ", &host);
      for ip in ips.iter() {
        println!("IP address: {}", ip);
      }

      
      let socket_addr = SocketAddr::new(ips[0], port);

      // TCPコネクションを構築
      let mut stream = match TcpStream::connect(socket_addr) {
          Ok(stream) => stream,
          Err(_e) => {
            return Err(Error::Network("Failed to connect to server".to_string()));
          }
      };

      // 非ブロッキングモードを設定
      match stream.set_nonblocking(true) {
          Ok(_) => {}
          Err(_e) => {
            return Err(Error::Network("Failed to set non-blocking mode".to_string()));
          }
      }

      // HTTPリクエストを作成
      let request_line: String = format!("GET {} HTTP/1.1\r\nHost: {}\r\n", path, host);

      let headers: Vec<String> = vec![
          format!("Host: {}", host),
          "Connection: close".to_string(),
          "User-Agent: RustyHttp/0.1".to_string(),
          "Accept: */*".to_string(),
      ];  

      let request: String = format!("{}\r\n{}\r\n", request_line, headers.join("\r\n"));

      println!("----Request:\r\n{}------", request);

      match stream.write_all(request.as_bytes()) {
          Ok(_bytes) => (),
          Err(_e) => {
            return Err(Error::Network("Failed to write to server".to_string()));
          }
      };
      stream.shutdown(std::net::Shutdown::Write).ok();

      let mut received = Vec::new();

      loop {
          let mut buffer = [0u8; 4096];
          match stream.read(&mut buffer) {
              Ok(0) => {
                println!("Connection closed by server");
                break;
              }
              Ok(n) => received.extend_from_slice(&buffer[..n]),
              Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // マニュアルでのポーリング実装
                std::thread::sleep(std::time::Duration::from_millis(50));
                continue;
              }
              Err(e) => {
                eprintln!("read error: {:?}", e); // エラーの種類を出力
                return Err(Error::Network("Failed to read from server".to_string()));
              }
          };
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