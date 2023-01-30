use actix_web::{web, HttpResponse};
use serde::{Deserialize};

use crate::{database::Database};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct UserLoginRequest {
    pub username: String,
    pub password: String
}

pub async fn login(user: web::Json<UserLoginRequest>, database: web::Data<Database>) -> HttpResponse {
    let user_entry = match database.get(&user.username) {
        Ok(value) => value,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match user.password == user_entry.password {
        true => HttpResponse::Ok().finish(),
        false => HttpResponse::BadRequest().finish(),
    }
}
