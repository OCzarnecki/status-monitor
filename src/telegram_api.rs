use crate::config::Config;

use std::sync::Arc;
use tokio::task;

pub async fn fire_message(cfg: Arc<Config>, msg: &str) {
    let local_cfg = Arc::new(cfg);
    let local_msg = String::from(msg);
    task::spawn(async move {
        let client = reqwest::Client::new();
        let params = [("chat_id", local_cfg.chat_id.as_str()), ("text", &local_msg)];
        match client.get(&format!("https://api.telegram.org/bot{}/sendMessage", &local_cfg.bot_token))
              .form(&params)
              .send()
              .await {
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
