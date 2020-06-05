
use actix_web::{Responder};

pub async fn get_venues() -> impl Responder {
    format!("GET LIST")
}

pub async fn add_venue() -> impl Responder {
    format!("POST ONE")
}

pub async fn get_venue_by_id() -> impl Responder {
    format!("GET ONE")
}

pub async fn edit_venue() -> impl Responder {
    format!("EDIT ONE")
}

pub async fn delete_venue() -> impl Responder {
    format!("DELETE ONE")
}