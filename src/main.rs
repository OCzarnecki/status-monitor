#[macro_use]
extern crate serde_derive;

mod config;
mod endpoint;
mod telegram_api;
mod timeouts;

use actix_web::web;
use std::sync::Mutex;

use crate::config::Config;
use crate::timeouts::Timeouts;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let cfg_webdata = web::Data::new(Config::load().unwrap_or_else(|e| panic!("FATAL: {}", e)));
    let timeouts_webdata = web::Data::new(Mutex::new(Timeouts::from_config(cfg_webdata.clone().into_inner())));
    endpoint::spawn_server(cfg_webdata, timeouts_webdata).await
}
