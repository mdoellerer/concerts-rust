use super::models::venues::{Venue, NewVenue, InputVenue, UpdateVenue};
use super::schema::venues::dsl::*;
use super::Pool;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse };
use diesel::dsl::{delete, insert_into};
use std::vec::Vec;

// Handler for GET /venues/
pub async fn get_venues(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_all_venues(db))
        .await
        .map(|venue| HttpResponse::Ok().json(venue))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_get_all_venues(db: web::Data<Pool>) -> Result<Vec<Venue>, diesel::result::Error> {
    let conn = db.get().unwrap();
    let items = venues.load::<Venue>(&conn)?;
    Ok(items)
}

// Handler for GET /venues/{id}
pub async fn get_venue_by_id(db: web::Data<Pool>, venue_id: web::Path<i64>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_venue_by_id(db, venue_id.into_inner()))
        .await
        .map(|venue| HttpResponse::Ok().json(venue))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_get_venue_by_id(db: web::Data<Pool>, venue_id: i64) -> Result<Venue, diesel::result::Error> {
    let conn = db.get().unwrap();
    venues.find(venue_id).get_result::<Venue>(&conn)    
}

// Handler for POST /venues/
pub async fn add_venue(db: web::Data<Pool>, item: web::Json<InputVenue>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_add_single_venue(db, item))
        .await
        .map(|venue| HttpResponse::Ok().json(venue))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_add_single_venue(db: web::Data<Pool>, item: web::Json<InputVenue>) -> Result<Venue, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_venue = NewVenue {
        name: &item.name,
        city: &item.city,
        country: &item.country,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(venues).values(&new_venue).get_result(&conn)?;
    Ok(res)
}

// Handler for PATCH /venues/{id}
pub async fn edit_venue(db: web::Data<Pool>, venue_id: web::Path<i64>, item: web::Json<InputVenue>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_edit_single_venue(db, venue_id.into_inner(), item))
        .await
        .map(|venue| HttpResponse::Created().json(venue))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_edit_single_venue(db: web::Data<Pool>, venue_id: i64, item: web::Json<InputVenue>) -> Result<Venue, diesel::result::Error> {
    let conn = db.get().unwrap();
    let venue = venues.find(venue_id).get_result::<Venue>(&conn)?;
    let update = UpdateVenue {
        name: &item.name,
        city: &item.city,
        country: &item.country,
        updated_at: chrono::Local::now().naive_local(),
    };   
    let res = diesel::update(&venue).set(&update).get_result(&conn)?;
    Ok(res)
}

// Handler for DELETE /venues/{id}
pub async fn delete_venue(db: web::Data<Pool>, venue_id: web::Path<i64>) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_delete_single_venue(db, venue_id.into_inner()))
            .await
            .map(|venue| HttpResponse::Ok().json(venue))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_delete_single_venue(db: web::Data<Pool>, venue_id: i64) -> Result<String, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(venues.find(venue_id)).execute(&conn)?;
    let message = format!("Total of deletions: {}", count.to_string());
    Ok(message)
}