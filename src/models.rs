use crate::schema::*;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Artist {
    pub id : i32,
    pub name : String,
    pub country : String,
    pub created_at : chrono::NaiveDateTime,
    pub updated_at : chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "artists"]
pub struct NewArtist<'a> {
    pub name: &'a str,
    pub country: &'a str,
    pub created_at : chrono::NaiveDateTime,
    pub updated_at : chrono::NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Venue {
    pub id : i32,
    pub name : String,
    pub city : String,
    pub country : String,
    pub created_at : chrono::NaiveDateTime,
    pub updated_at : chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "venues"]
pub struct NewVenue<'a> {
    pub name: &'a str,
    pub city: &'a str,
    pub country: &'a str,
    pub created_at : chrono::NaiveDateTime,
    pub updated_at : chrono::NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ConcertType {
    pub id : i32,
    pub dexcription : String,
    pub created_at : chrono::NaiveDateTime,
    pub updated_at : chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "concert_types"]
pub struct NewConcertType<'a> {
    pub description: &'a str,
    pub created_at : chrono::NaiveDateTime,
    pub updated_at : chrono::NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Concert {
    pub id : i32,
    pub concert_date : chrono::NaiveDateTime,
    pub setlist : String,
    pub artist_id : i32,
    pub venue_id : i32,
    pub concert_type_id : i32,
    pub created_at : chrono::NaiveDateTime,
    pub updated_at : chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "concerts"]
pub struct NewConcert<'a> {
    pub concert_date: chrono::NaiveDateTime,,
    pub setlist: &'a str,
    pub artist_id : i32,
    pub venue_id : i32,
    pub concert_type_id : i32,
    pub created_at : chrono::NaiveDateTime,
    pub updated_at : chrono::NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id : i32,
    pub name : String,
    pub email : String,
    pub email_verified_at : chrono::NaiveDateTime,
    pub password : String,
    pub remember_token : String,
    pub created_at : chrono::NaiveDateTime,
    pub updated_at : chrono::NaiveDateTime,
    pub api_token : String,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub email_verified_at : chrono::NaiveDateTime,
    pub password : &'a str,
    pub remember_token : &'a str,
    pub created_at : chrono::NaiveDateTime,
    pub updated_at : chrono::NaiveDateTime,
    pub api_token : &'a str,
}