use std::sync::Arc;
use std::time::Duration;

use crate::Config;
use crate::telegram_api;

pub async fn start_daemon(cfg_other: Arc<Config>) -> tokio::task::JoinHandle<()> {
    let cfg = cfg_other.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60 * 60 * 24));
        loop {
            interval.tick().await;
            telegram_api::fire_message(cfg.clone(), "Status Monitor is still alive!").await;
        }
    })
}
