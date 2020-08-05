#[macro_use]
extern crate serde_derive;

mod config;
mod endpoint;
mod telegram_api;

use std::sync::Arc;

use crate::config::Config;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let cfg_arc = Arc::new(Config::load().unwrap_or_else(|e| panic!("FATAL: {}", e)));
    endpoint::spawn_server(cfg_arc).await
}
