use crate::config::Config;

pub async fn send_message(cfg: Config, msg: String) {
    let resp = reqwest::get(
        &format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
            cfg.bot_token(),
            cfg.chat_id(),
            msg
        )
    ).await;
    println!("{:#?}", resp);
}
