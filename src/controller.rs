use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio;

use crate::{Config, Timeouts};
use crate::config::Service;
use crate::telegram_api;

pub async fn start_daemon(cfg_other: Arc<Config>, timeouts_other: Arc<Mutex<Timeouts>>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        let cfg = cfg_other.clone();
        let timeouts = timeouts_other.clone();
        loop {
            interval.tick().await;
            println!("Checking service state");

            let failed: Vec<Service>;
            {
                let mut mut_timeouts = timeouts.lock().unwrap();
                failed = mut_timeouts.get_failed();
            }

            send_failure_report(cfg.clone(), &failed).await;
        }
    })
}

async fn send_failure_report(cfg: Arc<Config>, failed_services: &Vec<Service>) {
    if failed_services.is_empty() {
        return
    }
    let mut message = "The following services failed to report:".to_owned();
    for s in failed_services {
        message += &format!("\n  - {}", s.name);
    }
    telegram_api::fire_message(cfg, &message).await;
}
