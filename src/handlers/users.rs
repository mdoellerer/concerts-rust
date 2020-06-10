use super::models::users::{User, NewUser, InputUser, UpdateUser};
use super::schema::users::dsl::*;
use super::Pool;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse };
use diesel::dsl::{insert_into};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;



// Handler for POST /login/
pub async fn user_login(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_user_login(db, item))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_user_login(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let user = users.filter(email.eq(&item.email)).get_result::<User>(&conn)?;
    let api_token_value = generate_api_token();

    let api_token_option = match user.password == item.password {
        true => Some(&api_token_value[..]),
        false => None,
    };
    db_update_user_token(db, &user, api_token_option)
}

fn generate_api_token() -> String {
    let rand_string: String = 
        thread_rng()
        .sample_iter(&Alphanumeric)
        .take(60)
        .collect();

    rand_string
}

// Handler for POST /logout/
pub async fn user_logout(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_user_logout(db, item))
        .await
        .map(|artist| HttpResponse::Created().json(artist))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_user_logout(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let user = users.filter(email.eq(&item.email)).get_result::<User>(&conn)?;

    db_update_user_token(db, &user, None)
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
    let new_user = NewUser {
        name: &item.name,
        email: &item.email,
        email_verified_at: chrono::Local::now().naive_local(),
        password: &item.password,
        remember_token: None,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
        api_token: None,
    };
    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}
