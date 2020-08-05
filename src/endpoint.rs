use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

use std::sync::Arc;

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
async fn index(info: web::Path<Params>, state: web::Data<Arc<Config>>) -> impl Responder {
    crate::telegram_api::send_message(&state, &info.text).await; // TODO this should not block
    HttpResponse::Ok()
        .json(Reply {
            ok: true,
            next_checkin: 42
        })
}

pub async fn spawn_server(cfg: Arc<Config>) -> std::io::Result<()> {
    HttpServer::new(move || {
        let local_cfg = Arc::clone(&cfg);
        App::new()
            .data(local_cfg)
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
