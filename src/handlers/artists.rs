use super::models::artists::{Artist, NewArtist, InputArtist, UpdateArtist};
use super::schema::artists::dsl::*;
use super::Pool;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse };
use diesel::dsl::{delete, insert_into};
use std::vec::Vec;


// Handler for GET /artists
pub async fn get_artists(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_all_artists(db))
        .await
        .map(|artist| HttpResponse::Ok().json(artist))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_get_all_artists(db: web::Data<Pool>) -> Result<Vec<Artist>, diesel::result::Error> {
    let conn = db.get().unwrap();
    let items = artists.load::<Artist>(&conn)?;
    Ok(items)
}

// Handler for GET /artists/{id}
pub async fn get_artist_by_id(db: web::Data<Pool>,artist_id: web::Path<i64>,) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_artist_by_id(db, artist_id.into_inner()))
            .await
            .map(|artist| HttpResponse::Ok().json(artist))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_get_artist_by_id(db: web::Data<Pool>, artist_id: i64) -> Result<Artist, diesel::result::Error> {
    let conn = db.get().unwrap();
    artists.find(artist_id).get_result::<Artist>(&conn)
}

// Handler for POST /artists
pub async fn add_artist(db: web::Data<Pool>, item: web::Json<InputArtist>,) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_add_single_artist(db, item))
        .await
        .map(|artist| HttpResponse::Created().json(artist))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_add_single_artist(db: web::Data<Pool>, item: web::Json<InputArtist>,) -> Result<Artist, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_artist = NewArtist {
        name: &item.name,
        country: &item.country,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(artists).values(&new_artist).get_result(&conn)?;
    Ok(res)
}

// Handler for PATCH /artists/{id}
pub async fn edit_artist(db: web::Data<Pool>, artist_id: web::Path<i64>, item: web::Json<InputArtist>,) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_edit_single_artist(db, artist_id.into_inner(), item))
        .await
        .map(|artist| HttpResponse::Created().json(artist))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_edit_single_artist(db: web::Data<Pool>, artist_id: i64, item: web::Json<InputArtist>,) -> Result<Artist, diesel::result::Error> {
    let conn = db.get().unwrap();
    let artist = artists.find(artist_id).get_result::<Artist>(&conn)?;
    let update = UpdateArtist {
        name: &item.name,
        country: &item.country,
        updated_at: chrono::Local::now().naive_local(),
    };    
    let res = diesel::update(&artist).set(&update).get_result(&conn)?;    
    Ok(res)
}

// Handler for DELETE /artists/{id}
pub async fn delete_artist(db: web::Data<Pool>, artist_id: web::Path<i64>,) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_delete_single_artist(db, artist_id.into_inner()))
            .await
            .map(|artist| HttpResponse::Ok().json(artist))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_delete_single_artist(db: web::Data<Pool>, artist_id: i64) -> Result<String, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(artists.find(artist_id)).execute(&conn)?;
    let message = format!("Total of deletions: {}", count.to_string());
    Ok(message)
}