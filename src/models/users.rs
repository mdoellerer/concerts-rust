use crate::schema::users;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct User {
    pub id : i64,
    pub name : String,
    pub email : String,
    pub email_verified_at : Option<chrono::NaiveDateTime>,
    pub password : String,
    pub remember_token : Option<String>,
    pub created_at : chrono::NaiveDateTime,
    pub updated_at : chrono::NaiveDateTime,
    pub api_token : Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub email_verified_at : chrono::NaiveDateTime,
    pub password : &'a str,
    pub remember_token : Option<&'a str>,
    pub created_at : chrono::NaiveDateTime,
    pub updated_at : chrono::NaiveDateTime,
    pub api_token : Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(AsChangeset)]
#[table_name="users"]
pub struct UpdateUser<'a> {
    pub updated_at : chrono::NaiveDateTime,
    pub api_token: Option<&'a str>,
}