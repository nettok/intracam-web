use crate::app_state::AppState;

use actix_web::{get, post, web, Error, HttpResponse , Responder, Result};
use futures::StreamExt;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(ping)
        .service(photo_upload)
    ;
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[post("/photos")]
async fn photo_upload(app_state: web::Data<AppState>, mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();

    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let mut photos = app_state.photos.lock().unwrap();
    photos.insert(0, bytes.freeze());
    photos.truncate(12);

    Ok(HttpResponse::Created().finish())
}
