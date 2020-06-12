use super::models::users::{User, NewUser, UpdateUser };
use super::schema::users::dsl::*;
use super::Pool;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse };
use diesel::dsl::{insert_into};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub message: String,
    pub email: String,
    pub token: String,
}


// Handler for POST /login/
pub async fn user_login(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_user_login(db, item))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_user_login(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<LoginResponse, diesel::result::Error> {
    let conn = db.get().unwrap();
    let user = users.filter(email.eq(&item.email)).get_result::<User>(&conn)?;
    
    let api_token_value = generate_api_token();
    let hash_pw = hash_password(&item.password);

    match user.password == hash_pw {
        true => {
                
            let user = db_update_user_token(db, &user, Some(&api_token_value[..])).unwrap();
            let message = LoginResponse{
                message: format! ("Welcome, {}", user.name),
                email: user.email,
                token: user.api_token.unwrap(),
            };
            Ok(message)
        }
        false => {
            Ok(unauthorized_access())
        }
    }
}

fn db_update_user_token(db: web::Data<Pool>, user: &User, value: Option<&str>)-> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let update = UpdateUser {
        updated_at: chrono::Local::now().naive_local(),
        api_token: value,
    };    
    let res = diesel::update(user).set(&update).get_result(&conn)?;    
    Ok(res)
}

fn hash_password(pass: &str) -> String {
    let mut sha = Sha256::new();
    sha.input_str(pass);
    sha.result_str()
}

fn generate_api_token() -> String {
    let rand_string: String = 
        thread_rng()
        .sample_iter(&Alphanumeric)
        .take(60)
        .collect();

    rand_string
}

fn unauthorized_access() -> LoginResponse {
    LoginResponse{
        message: String::from("Wrong access information, wrong email or password!"),
        email: String::from(""),
        token: String::from(""),
    }
}


// Handler for POST /logout/
pub async fn user_logout(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_user_logout(db, item))
        .await
        .map(|artist| HttpResponse::Created().json(artist))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_user_logout(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<LoginResponse, diesel::result::Error> {
    let conn = db.get().unwrap();
    let user = users.filter(email.eq(&item.email)).get_result::<User>(&conn)?;

    let user = db_update_user_token(db, &user, None).unwrap();
    let message = LoginResponse{
        message: format! ("Log out was successful , {}", user.name),
        email: user.email,
        token: String::from(""),
    };
    Ok(message)
}


// Handler for POST /register/
pub async fn add_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_add_single_user(db, item))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_add_single_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let hash_pw = hash_password(&item.password);

    let new_user = NewUser {
        name: &item.name,
        email: &item.email,
        email_verified_at: chrono::Local::now().naive_local(),
        password: &hash_pw,
        remember_token: None,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
        api_token: None,
    };
    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}
