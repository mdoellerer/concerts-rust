use super::models::{Concert, NewConcert, InputConcert, UpdateConcert};
use super::schema::concerts::dsl::*;
use super::Pool;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse };
use diesel::dsl::{delete, insert_into};
use std::vec::Vec;

// Handler for GET /concerts/
pub async fn get_concerts(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_all_concerts(db))
        .await
        .map(|concert| HttpResponse::Ok().json(concert))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_get_all_concerts(db: web::Data<Pool>) -> Result<Vec<Concert>, diesel::result::Error> {
    let conn = db.get().unwrap();
    let items = concerts.load::<Concert>(&conn)?;
    Ok(items)
}

// Handler for GET /concerts/{id}
pub async fn get_concert_by_id(db: web::Data<Pool>, concert_id: web::Path<i64>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_concert_by_id(db, concert_id.into_inner()))
        .await
        .map(|concert| HttpResponse::Ok().json(concert))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_get_concert_by_id(db: web::Data<Pool>, concert_id: i64) -> Result<Concert, diesel::result::Error> {
    let conn = db.get().unwrap();
    concerts.find(concert_id).get_result::<Concert>(&conn)    
}

// Handler for POST /concerts/
pub async fn add_concert(db: web::Data<Pool>, item: web::Json<InputConcert>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_add_single_concert(db, item))
        .await
        .map(|concert| HttpResponse::Ok().json(concert))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_add_single_concert(db: web::Data<Pool>, item: web::Json<InputConcert>) -> Result<Concert, diesel::result::Error> {
    let conn = db.get().unwrap();
    let concert_date_value = chrono::NaiveDate::parse_from_str(&item.concert_date, "%Y-%m-%d").unwrap();
    let new_concert = NewConcert {
        concert_date: concert_date_value,
        setlist: &item.setlist,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
        concert_type_id: item.concert_type_id,
        artist_id: item.artist_id,
        venue_id: item.venue_id,
    };
    let res = insert_into(concerts).values(&new_concert).get_result(&conn)?;
    Ok(res)
}

// Handler for PATCH /concerts/{id}
pub async fn edit_concert(db: web::Data<Pool>, concert_id: web::Path<i64>, item: web::Json<InputConcert>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_edit_single_concert(db, concert_id.into_inner(), item))
        .await
        .map(|concert| HttpResponse::Created().json(concert))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_edit_single_concert(db: web::Data<Pool>, concert_id: i64, item: web::Json<InputConcert>) -> Result<Concert, diesel::result::Error> {
    let conn = db.get().unwrap();
    let concert = concerts.find(concert_id).get_result::<Concert>(&conn)?;
    let concert_date_value = chrono::NaiveDate::parse_from_str(&item.concert_date, "%Y-%m-%d").unwrap();
    let update = UpdateConcert {
        concert_date: concert_date_value,
        setlist: &item.setlist,
        updated_at: chrono::Local::now().naive_local(),
        concert_type_id: item.concert_type_id,
        artist_id: item.artist_id,
        venue_id: item.venue_id,
    };   
    let res = diesel::update(&concert).set(&update).get_result(&conn)?;
    Ok(res)
}

// Handler for DELETE /concerts/{id}
pub async fn delete_concert(db: web::Data<Pool>, concert_id: web::Path<i64>) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_delete_single_concert(db, concert_id.into_inner()))
            .await
            .map(|concert| HttpResponse::Ok().json(concert))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_delete_single_concert(db: web::Data<Pool>, concert_id: i64) -> Result<String, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(concerts.find(concert_id)).execute(&conn)?;
    let message = format!("Total of deletions: {}", count.to_string());
    Ok(message)
}