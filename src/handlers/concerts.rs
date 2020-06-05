
use actix_web::{Responder};

pub async fn get_concerts() -> impl Responder {
    format!("GET LIST")
}

pub async fn add_concert() -> impl Responder {
    format!("POST ONE")
}

pub async fn get_concert_by_id() -> impl Responder {
    format!("GET ONE")
}

pub async fn edit_concert() -> impl Responder {
    format!("EDIT ONE")
}

pub async fn delete_concert() -> impl Responder {
    format!("DELETE ONE")
}