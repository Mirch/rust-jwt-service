use std::net::TcpListener;

use rust_jwt_service::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    run(listener)?.await
}
