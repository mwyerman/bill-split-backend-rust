pub mod data_config;
pub mod server_config;

#[derive(Clone)]
pub struct Config {
    pub data: data_config::DataConfig,
    pub app: server_config::ServerConfig
}


impl Config {
    pub fn new() -> Config {
        Config {
            data: data_config::DataConfig::new(),
            app: server_config::ServerConfig::new()
        }
    }
}
