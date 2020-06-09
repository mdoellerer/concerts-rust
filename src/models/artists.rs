use crate::schema::artists;
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