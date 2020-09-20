use actix_web::{get, post, web, HttpResponse, Result};
use askama::Template;
use serde::Deserialize;

use qrcode::QrCode;
use image::{Luma, ImageEncoder, ColorType};
use base64;

#[derive(Template)]
#[template(path = "qr.html")]
struct QR {
    base64_png_code: Option<String>
}

#[derive(Deserialize)]
pub struct QRForm {
    ssid: String,
    pass: String,
}

#[get("/qr")]
pub async fn qr() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(
        (QR { base64_png_code: None }).render().unwrap()
    ))
}

#[post("/qr")]
pub async fn qr_post(qr_form: web::Form<QRForm>) -> Result<HttpResponse> {
    let data: Vec<u8> = format!("{},{}", qr_form.ssid, qr_form.pass).into_bytes();
    let code = QrCode::new(data).unwrap();
    let image = code.render::<Luma<u8>>().build();
    let (width, height) = image.dimensions();
    let mut png: Vec<u8> = vec!();
    let png_encoder = image::png::PngEncoder::new(&mut png);
    png_encoder.write_image(image.as_ref(), width, height, ColorType::L8).unwrap();
    let png_base64 = base64::encode(png);
    Ok(HttpResponse::Ok().content_type("text/html").body(
        (QR { base64_png_code: Some(png_base64) }).render().unwrap()
    ))
}
