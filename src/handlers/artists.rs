use actix_web::{Responder};

pub async fn get_artists() -> impl Responder {
    format!("GET LIST")
}

pub async fn add_artist() -> impl Responder {
    format!("POST ONE")
}

pub async fn get_artist_by_id() -> impl Responder {
    format!("GET ONE")
}

pub async fn edit_artist() -> impl Responder {
    format!("EDIT ONE")
}

pub async fn delete_artist() -> impl Responder {
    format!("DELETE ONE")
}