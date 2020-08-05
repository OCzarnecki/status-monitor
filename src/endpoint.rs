use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

use crate::config::Config;

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
async fn index(info: web::Path<Params>, state: web::Data<Config>) -> impl Responder {
    crate::telegram_api::fire_message(state.into_inner(), &info.text).await;
    HttpResponse::Ok()
        .json(Reply {
            ok: true,
            next_checkin: 42
        })
}

pub async fn spawn_server(cfg: web::Data<Config>) -> std::io::Result<()> {
    HttpServer::new(move || {
        let local_cfg = cfg.clone();
        App::new()
            .app_data(local_cfg)
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
