use actix_web::web::Bytes;
use chrono::{DateTime, Utc};
use std::sync::Mutex;

pub struct AppState {
    pub photos: Mutex<Vec<Photo>>
}

pub struct Photo {
    pub image: Bytes,
    pub timestamp: DateTime<Utc>,
}
