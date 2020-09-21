use actix_web::web::Bytes;
use std::sync::Mutex;

pub struct AppState {
    pub photos: Mutex<Vec<Bytes>>
}
