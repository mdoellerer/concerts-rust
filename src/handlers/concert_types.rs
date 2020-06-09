use super::models::concert_types::{ConcertType, NewConcertType, InputConcertType, UpdateConcertType};
use super::schema::concert_types::dsl::*;
use super::Pool;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse };
use diesel::dsl::{delete, insert_into};
use std::vec::Vec;

// Handler for GET /concert_types/
pub async fn get_concert_types(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_all_concert_types(db))
        .await
        .map(|concert_type| HttpResponse::Ok().json(concert_type))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_get_all_concert_types(db: web::Data<Pool>) -> Result<Vec<ConcertType>, diesel::result::Error> {
    let conn = db.get().unwrap();
    let items = concert_types.load::<ConcertType>(&conn)?;
    Ok(items)
}

// Handler for GET /concert_types/{id}
pub async fn get_concert_type_by_id(db: web::Data<Pool>, concert_type_id: web::Path<i64>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_concert_type_by_id(db, concert_type_id.into_inner()))
        .await
        .map(|concert_type| HttpResponse::Ok().json(concert_type))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_get_concert_type_by_id(db: web::Data<Pool>, concert_type_id: i64) -> Result<ConcertType, diesel::result::Error> {
    let conn = db.get().unwrap();
    concert_types.find(concert_type_id).get_result::<ConcertType>(&conn)    
}

// Handler for POST /concert_types/
pub async fn add_concert_type(db: web::Data<Pool>, item: web::Json<InputConcertType>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_add_single_concert_type(db, item))
        .await
        .map(|concert_type| HttpResponse::Ok().json(concert_type))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_add_single_concert_type(db: web::Data<Pool>, item: web::Json<InputConcertType>) -> Result<ConcertType, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_concert_type = NewConcertType {
        description: &item.description,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(concert_types).values(&new_concert_type).get_result(&conn)?;
    Ok(res)
}

// Handler for PATCH /concert_types/{id}
pub async fn edit_concert_type(db: web::Data<Pool>, concert_type_id: web::Path<i64>, item: web::Json<InputConcertType>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_edit_single_concert_type(db, concert_type_id.into_inner(), item))
        .await
        .map(|concert_type| HttpResponse::Created().json(concert_type))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_edit_single_concert_type(db: web::Data<Pool>, concert_type_id: i64, item: web::Json<InputConcertType>) -> Result<ConcertType, diesel::result::Error> {
    let conn = db.get().unwrap();
    let concert_type = concert_types.find(concert_type_id).get_result::<ConcertType>(&conn)?;
    let update = UpdateConcertType {
        description: &item.description,
        updated_at: chrono::Local::now().naive_local(),
    };   
    let res = diesel::update(&concert_type).set(&update).get_result(&conn)?;
    Ok(res)
}

// Handler for DELETE /concert_types/{id}
pub async fn delete_concert_type(db: web::Data<Pool>, concert_type_id: web::Path<i64>) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_delete_single_concert_type(db, concert_type_id.into_inner()))
            .await
            .map(|concert_type| HttpResponse::Ok().json(concert_type))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_delete_single_concert_type(db: web::Data<Pool>, concert_type_id: i64) -> Result<String, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(concert_types.find(concert_type_id)).execute(&conn)?;
    let message = format!("Total of deletions: {}", count.to_string());
    Ok(message)
}