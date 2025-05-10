extern crate alloc;

pub mod http;
pub mod response;
pub mod request;
pub mod error;
pub mod header;

use crate::http::HttpClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HttpClient::new();

    // Example usage:
    let host = "example.com".to_string();
    let port = 80;
    let path = "/".to_string();

    println!("Making GET request to http://{}:{}{}", host, port, path);

    match client.get(host.clone(), port, path.clone()) {
        Ok(response) => {
            // Assuming HttpResponse has a way to be displayed or converted to string
            // For now, let's just print a success message.
            // We'll need to implement Display or a method to get the body for HttpResponse later.
            println!("Successfully received response from {}:{}{}", host, port, path);
            println!("Response: {:?}", response.body());
        }
        Err(e) => {
            eprintln!("Error during GET request: {}", e);
        }
    }

    Ok(())
}