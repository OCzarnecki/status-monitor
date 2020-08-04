#[macro_use]
extern crate serde_derive;

mod config;
mod telegram_api;

use crate::config::Config;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    text: String
}

#[derive(Serialize)]
struct Reply {
    ok: bool,
    next_checkin: u32
}

#[get("/{text}/checkin")]
async fn index(info: web::Path<Params>) -> impl Responder {
    let cfg = Config::load().unwrap();
    crate::telegram_api::send_message(cfg, &info.text).await; // TODO this should not block
    HttpResponse::Ok()
        .json(Reply {
            ok: true,
            next_checkin: 42
        })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
