mod generated_static_files;
mod index;
mod qr;

use crate::generated_static_files::generate;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files;
use env_logger::Env;

#[macro_use]
extern crate log;

struct AppState {
}

#[post("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let app_state = web::Data::new(AppState {
    });

    HttpServer::new(move || {
        let generated = generate();

        App::new()
            .app_data(app_state.clone())
            .service(actix_web_static_files::ResourceFiles::new(
                "/static", generated,
            ))
            .service(ping)
            .service(index::index)
            .service(qr::qr)
            .service(qr::qr_post)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
