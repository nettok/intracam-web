use actix_web::{get, HttpResponse, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[get("/")]
pub async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(
        Index.render().unwrap()
    ))
}
