// Add configuration for server address and symbols
pub struct Config {
    pub server_address: String,
    pub symbols: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            server_address: "ws://localhost:8080".to_string(),
            symbols: vec!["AAPL".to_string(), "GOOGL".to_string()],
        }
    }
}