use crate::schema::*;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Artist {
    pub id : i64,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct InputArtist {
    pub name: String,
    pub country: String,
}

#[derive(AsChangeset)]
#[table_name="artists"]
pub struct UpdateArtist<'a> {
    pub name: &'a str,
    pub country: &'a str,
    pub updated_at : chrono::NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Venue {
    pub id : i64,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct InputVenue {
    pub name: String,
    pub city: String,
    pub country: String,
}

#[derive(AsChangeset)]
#[table_name="venues"]
pub struct UpdateVenue<'a> {
    pub name: &'a str,
    pub city: &'a str,
    pub country: &'a str,
    pub updated_at : chrono::NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct ConcertType {
    pub id : i64,
    pub description : String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct InputConcertType {
    pub description: String,
}

#[derive(AsChangeset)]
#[table_name="concert_types"]
pub struct UpdateConcertType<'a> {
    pub description: &'a str,
    pub updated_at : chrono::NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Concert {
    pub id : i64,
    pub concert_date : chrono::NaiveDate,
    pub setlist : Option<String>,
    pub created_at : chrono::NaiveDateTime,
    pub updated_at : chrono::NaiveDateTime,       
    pub concert_type_id : i64,
    pub artist_id : i64,
    pub venue_id : i64,
}

#[derive(Insertable, Debug)]
#[table_name = "concerts"]
pub struct NewConcert<'a> {
    pub concert_date: chrono::NaiveDate,
    pub setlist: &'a str,    
    pub created_at : chrono::NaiveDateTime,
    pub updated_at : chrono::NaiveDateTime,
    pub concert_type_id : i64,
    pub artist_id : i64,
    pub venue_id : i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputConcert {
    pub concert_date: String,
    pub setlist: String,
    pub artist_id : i64,
    pub venue_id : i64,
    pub concert_type_id : i64,
}

#[derive(AsChangeset)]
#[table_name="concerts"]
pub struct UpdateConcert<'a> {
    pub concert_date: chrono::NaiveDate,
    pub setlist: &'a str,    
    pub updated_at : chrono::NaiveDateTime,
    pub concert_type_id : i64,
    pub artist_id : i64,
    pub venue_id : i64,    
}


#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct User {
    pub id : i64,
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