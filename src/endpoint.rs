use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::sync::Mutex;

use crate::config::Config;
use crate::telegram_api;
use crate::timeouts::{Timeouts, CheckInResult};

#[derive(Deserialize)]
struct Params {
    handle: String
}

#[derive(Serialize)]
struct ReplyOk {
    ok: bool,
    next_checkin: u32
}

#[derive(Serialize)]
struct ReplyErr {
    ok: bool,
    message: String
}

#[get("/{handle}/checkin")]
async fn index(info: web::Path<Params>,
               cfg: web::Data<Config>,
               timeouts: web::Data<Mutex<Timeouts>>) -> impl Responder {
    let handle = &info.handle;
    let mut timeouts = timeouts.lock().unwrap();
    match timeouts.check_in(handle) {
        Some(result) => {
            match result {
                CheckInResult::Normal => (),
                CheckInResult::Recovery => {
                    telegram_api::fire_message(cfg.into_inner(),
                        &format!("{} is back online!", timeouts.name(handle).unwrap())).await;
                    ()
                }
            };
            HttpResponse::Ok()
                .json(ReplyOk {
                    ok: true,
                    next_checkin: timeouts.interval(handle).unwrap()
                })
        }
        None => HttpResponse::NotFound()
                    .json(ReplyErr {
                        ok: false,
                        message: "No service with this handle.".to_string()
                    })
    }
}

pub async fn spawn_server(cfg: web::Data<Config>,
                          timeouts: web::Data<Mutex<Timeouts>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        let local_cfg = cfg.clone();
        let local_timeouts = timeouts.clone();
        App::new()
            .app_data(local_cfg)
            .app_data(local_timeouts)
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
