
use actix_web::{Responder};

pub async fn get_concert_types() -> impl Responder {
    format!("GET LIST")
}

pub async fn add_concert_type() -> impl Responder {
    format!("POST ONE")
}

pub async fn get_concert_type_by_id() -> impl Responder {
    format!("GET ONE")
}

pub async fn edit_concert_type() -> impl Responder {
    format!("EDIT ONE")
}

pub async fn delete_concert_type() -> impl Responder {
    format!("DELETE ONE")
}