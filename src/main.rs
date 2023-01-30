use rust_jwt_service::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = "localhost:8080";

    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
