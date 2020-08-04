#[macro_use]
extern crate serde_derive;

mod config;
mod telegram_api;

use crate::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::load()?;
    crate::telegram_api::send_message(cfg, "Now from submodule!".to_string()).await;
    Ok(())
}
