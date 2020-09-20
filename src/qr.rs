use actix_web::{get, post, web, HttpResponse, Result};
use askama::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "qr.html")]
struct QR;

#[derive(Deserialize)]
pub struct QRForm {
    ssid: String,
    pass: String,
}

#[get("/qr")]
pub async fn qr() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(
        QR.render().unwrap()
    ))
}

#[post("/qr")]
pub async fn qr_post(qr_form: web::Form<QRForm>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(
        QR.render().unwrap()
    ))
}
