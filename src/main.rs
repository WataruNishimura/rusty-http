extern crate alloc;

pub mod http;
pub mod response;
pub mod request;
pub mod error;
pub mod header;

use crate::http::HttpClient;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter URL (e.g., http://example.com/path): ");
    io::stdout().flush()?; // Ensure the prompt is displayed before reading input

    let mut url_str = String::new();
    io::stdin().read_line(&mut url_str)?;
    let url_str = url_str.trim();

    if !url_str.starts_with("http://") {
        eprintln!("Invalid URL format. Please start with http://");
        return Ok(()); // Or return an error
    }

    let without_scheme = &url_str[7..]; // Remove "http://"

    let (host_port_str, path_str) = match without_scheme.find('/') {
        Some(index) => {
            (&without_scheme[..index], &without_scheme[index..])
        }
        None => (without_scheme, "/"),
    };

    let (host, port_str_opt) = match host_port_str.rfind(':') {
        Some(index) if index > 0 && host_port_str.chars().nth(index-1) != Some('/') => { // Ensure ':' is for port, not in path if path was part of host_port_str
            (&host_port_str[..index], Some(&host_port_str[index + 1..]))
        }
        _ => (host_port_str, None),
    };

    let port: u16 = match port_str_opt {
        Some(p_str) => p_str.parse().map_err(|e| format!("Invalid port number: {}", e))?,
        None => 80,
    };

    let path = if path_str.is_empty() { "/" } else { path_str };

    let client = HttpClient::new();
    println!("Making GET request to http://{}:{}{}", host, port, path);

    match client.get(host.to_string(), port, path.to_string()) {
        Ok(response) => {
            println!("Successfully received response from {}:{}{}", host, port, path);
            println!("Status: {} {}", response.status_code(), response.reason_phrase());
            println!("Version: {}", response.version());
            println!("Headers:");
            for header in response.headers() {
                // Assuming Header has a way to be displayed or has fields
                 println!("  {}: {}", header.name, header.value); // Adjust if Header struct is different
            }
            println!("Body:\n{}", response.body());
        }
        Err(e) => {
            eprintln!("Error during GET request: {}", e);
        }
    }

    Ok(())
}