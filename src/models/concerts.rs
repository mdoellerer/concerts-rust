use crate::schema::concerts;
use serde::{Serialize, Deserialize};

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