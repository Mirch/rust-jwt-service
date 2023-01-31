use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::{
    database::Database,
    jwt::build_token,
    models::{User, UserRole},
};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct UserAuthRequest {
    pub username: String,
    pub password: String,
}

pub async fn register(
    user: web::Json<UserAuthRequest>,
    database: web::Data<Database>,
) -> HttpResponse {
    match database.get(&user.username) {
        Some(_) => return HttpResponse::BadRequest().finish(),
        None => {}
    };

    let user_entry = User {
        username: user.username.clone(),
        password: user.password.clone(),
        role: UserRole::Regular,
    };

    match database.insert(&user_entry) {
        Ok(_) => {}
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    build_jwt_response(user_entry.username, user_entry.role)
}

pub async fn login(
    user: web::Json<UserAuthRequest>,
    database: web::Data<Database>,
) -> HttpResponse {
    let user_entry = match database.get(&user.username) {
        Some(value) => value,
        None => return HttpResponse::NotFound().finish(),
    };

    if user.password != user_entry.password {
        return HttpResponse::BadRequest().finish();
    }

    build_jwt_response(user_entry.username, user_entry.role)
}

fn build_jwt_response(username: String, role: UserRole) -> HttpResponse {
    let token = match build_token(username, role, 60 * 60) {
        Ok(value) => value,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let token_response = serde_json::to_string_pretty(&json!({ "token": token })).unwrap();

    HttpResponse::Ok().json(token_response)
}
