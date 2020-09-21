mod generated_static_files;
mod api;
mod index;
mod qr;
mod photos;
mod app_state;

use crate::generated_static_files::generate;
use app_state::AppState;

use actix_web::{web, App, HttpServer};
use actix_web_static_files;
use actix_web::middleware::Logger;
use env_logger::Env;
use std::sync::Mutex;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let app_state = web::Data::new(AppState {
        photos: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        let generated = generate();

        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(actix_web_static_files::ResourceFiles::new(
                "/static", generated,
            ))
            .service(
                web::scope("/api")
                    .app_data(app_state.clone())
                    .configure(api::api_config)
            )
            .service(index::index)
            .service(qr::qr_form_page)
            .service(qr::qr_generate)
            .service(photos::photos)

    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
