use crate::schema::concert_types;
use serde::{Serialize, Deserialize};

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

#[derive(AsChangeset)]
#[table_name="concert_types"]
pub struct UpdateConcertType<'a> {
    pub description: &'a str,
    pub updated_at : chrono::NaiveDateTime,
}