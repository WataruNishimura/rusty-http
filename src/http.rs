extern crate alloc; 
use core::net::IpAddr;

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

      

      
    }
}