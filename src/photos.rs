use crate::app_state::AppState;

use actix_web::{get, web, HttpResponse, Result};
use askama::Template;
use base64;

#[derive(Template)]
#[template(path = "photos.html")]
struct Photos {
    base64_photos: Vec<String>
}

#[get("/photos")]
pub async fn photos(app_state: web::Data<AppState>) -> Result<HttpResponse> {
    let photos = app_state.photos.lock().unwrap();

    let base64_photos = photos.iter()
        .map(|photo| base64::encode(photo))
        .collect();

    Ok(HttpResponse::Ok().content_type("text/html").body(
        (Photos { base64_photos }).render().unwrap()
    ))
}
