extern crate serde_yaml;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub bot_token: String,
    pub chat_id: String,
    pub port: u16,
    pub services: HashMap<String, Service>
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Service {
    pub name: String,
    pub interval: u32
}

impl Config {
    pub fn load() -> Result<Config, String> {
        std::fs::File::open("status-monitor-config.yml")
            .map_err(|e| format!("Could not open config file: {:#}", e))
            .and_then(|f| serde_yaml::from_reader(f)
                .map_err(|e| format!("Could not parse config file: {:#}", e)))
    }
}
