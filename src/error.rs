use std::fmt;

#[derive(Debug)]
pub enum Error {
    Network(String),
    // We can add other error variants here later
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Network(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for Error {} // So it can be used with `Box<dyn std::error::Error>` etc.

// We can implement the constructor here
impl Error {
    pub fn network(message: String) -> Self {
        Error::Network(message)
    }
}