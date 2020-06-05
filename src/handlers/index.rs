use actix_web::{HttpResponse, Responder};

pub async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("<h2>Welcome to Concerts API v1!</h2>")
}