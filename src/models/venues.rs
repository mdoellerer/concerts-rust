use crate::schema::venues;
use serde::{Serialize, Deserialize};

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

#[derive(AsChangeset)]
#[table_name="venues"]
pub struct UpdateVenue<'a> {
    pub name: &'a str,
    pub city: &'a str,
    pub country: &'a str,
    pub updated_at : chrono::NaiveDateTime,
}