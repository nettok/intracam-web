use crate::app_state::AppState;

use actix_web::{get, web, HttpResponse, Result};
use askama::Template;
use base64;

#[derive(Template)]
#[template(path = "photos.html")]
struct Photos {
    photos: Vec<Photo>
}

struct Photo {
    base64_image: String,
    timestamp: String,
}


#[get("/photos")]
pub async fn photos(app_state: web::Data<AppState>) -> Result<HttpResponse> {
    let photos = app_state.photos.lock().unwrap();

    let photos = photos.iter()
        .map(|photo| {
            Photo {
                base64_image: base64::encode(&photo.image),
                timestamp: photo.timestamp.to_string()
            }
        })
        .collect();

    Ok(HttpResponse::Ok().content_type("text/html").body(
        (Photos { photos }).render().unwrap()
    ))
}
