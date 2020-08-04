extern crate serde_yaml;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub bot_token: String,
    pub chat_id: String,
    pub services: HashMap<String, Service>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub interval: u32
}

impl Config {
    pub fn load() -> Result<Config, String> {
        match std::fs::File::open("status-monitor-config.yml") {
            Ok(file) => {
                let parse_result: serde_yaml::Result<Config> = serde_yaml::from_reader(file);
                match parse_result {
                    Ok(cfg) => return Ok(cfg),
                    Err(error) => return Err(error.to_string())
                }
            },
            Err(error) => return Err(error.to_string())
        }
    }
}
