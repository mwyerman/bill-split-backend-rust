#[derive(Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn new() -> ServerConfig {
        ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 3000,
        }
    }
}
