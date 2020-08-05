use crate::config::Config;

use std::sync::Arc;
use tokio::task;

pub async fn fire_message(cfg: Arc<Config>, msg: &str) {
    let local_cfg = Arc::new(cfg);
    let local_msg = String::from(msg);
    task::spawn(async move {
        match reqwest::get(
            &format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
            &local_cfg.bot_token,
            &local_cfg.chat_id,
            local_msg
        )).await {
            Ok(response) => {
                if response.status() != reqwest::StatusCode::OK {
                    println!("Telegram API did not return status 200. Response body:{}"
                        , &response.text_with_charset("utf8").await.unwrap())
                }
            },
            Err(e) => println!("Failed to connect to Telegram API! Cause: {:?}", e)
        };
    });
}
