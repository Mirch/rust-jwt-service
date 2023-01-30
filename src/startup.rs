use crate::{routes::{health_check, login}, database::Database};
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let database = web::Data::new(Database::new());

    let server = HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/login", web::post().to(login))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
