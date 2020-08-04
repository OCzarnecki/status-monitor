extern crate serde_yaml;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    bot_token: String,
    chat_id: String,
    services: HashMap<String, Service>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    name: String,
    interval: u32
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

    pub fn bot_token(&self) -> String {
        "1273678131:AAF0Mp-NHk2EEwxDqnX9-2VLOMQdlPS_NfQ".to_string()
    }

    pub fn chat_id(&self) -> String {
        "292570719".to_string()
    }
}
