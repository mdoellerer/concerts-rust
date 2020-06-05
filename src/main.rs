
mod handlers;

use handlers::index;
use handlers::artists;
use handlers::venues;
use handlers::concerts;
use handlers::concert_types;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

        
    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index::welcome))

            .route("/artists", web::get().to(artists::get_artists))
            .route("/artists/", web::get().to(artists::get_artists))
            .route("/artists/{id}", web::get().to(artists::get_artist_by_id))
            .route("/artists/{id}/", web::get().to(artists::get_artist_by_id))
            .route("/artists", web::post().to(artists::add_artist))
            .route("/artists/", web::post().to(artists::add_artist))
            .route("/artists/{id}", web::patch().to(artists::edit_artist))
            .route("/artists/{id}/", web::patch().to(artists::edit_artist))
            .route("/artists/{id}", web::delete().to(artists::delete_artist))
            .route("/artists/{id}/", web::delete().to(artists::delete_artist))

            .route("/venues", web::get().to(venues::get_venues))
            .route("/venues/", web::get().to(venues::get_venues))
            .route("/venues/{id}", web::get().to(venues::get_venue_by_id))
            .route("/venues/{id}/", web::get().to(venues::get_venue_by_id))
            .route("/venues", web::post().to(venues::add_venue))
            .route("/venues/", web::post().to(venues::add_venue))
            .route("/venues/{id}", web::patch().to(venues::edit_venue))
            .route("/venues/{id}/", web::patch().to(venues::edit_venue))
            .route("/venues/{id}", web::delete().to(venues::delete_venue))
            .route("/venues/{id}/", web::delete().to(venues::delete_venue))

            .route("/concert_types", web::get().to(concert_types::get_concert_types))
            .route("/concert_types/", web::get().to(concert_types::get_concert_types))
            .route("/concert_types/{id}", web::get().to(concert_types::get_concert_type_by_id))
            .route("/concert_types/{id}/", web::get().to(concert_types::get_concert_type_by_id))
            .route("/concert_types", web::post().to(concert_types::add_concert_type))
            .route("/concert_types/", web::post().to(concert_types::add_concert_type))
            .route("/concert_types/{id}", web::patch().to(concert_types::edit_concert_type))
            .route("/concert_types/{id}/", web::patch().to(concert_types::edit_concert_type))
            .route("/concert_types/{id}", web::delete().to(concert_types::delete_concert_type))
            .route("/concert_types/{id}/", web::delete().to(concert_types::delete_concert_type))

            .route("/concerts", web::get().to(concerts::get_concerts))
            .route("/concerts/", web::get().to(concerts::get_concerts))
            .route("/concerts/{id}", web::get().to(concerts::get_concert_by_id))
            .route("/concerts/{id}/", web::get().to(concerts::get_concert_by_id))
            .route("/concerts", web::post().to(concerts::add_concert))
            .route("/concerts/", web::post().to(concerts::add_concert))
            .route("/concerts/{id}", web::patch().to(concerts::edit_concert))
            .route("/concerts/{id}/", web::patch().to(concerts::edit_concert))
            .route("/concerts/{id}", web::delete().to(concerts::delete_concert))
            .route("/concerts/{id}/", web::delete().to(concerts::delete_concert))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
