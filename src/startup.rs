use crate::{
    database::Database,
    routes::{get_secret_info, health_check, login, register},
};
use actix_web::{dev::Server, web, App, HttpServer};

pub fn run() -> Result<Server, std::io::Error> {
    let database = web::Data::new(Database::new());

    let server = HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            .route("/secret", web::get().to(get_secret_info))
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    Ok(server)
}
