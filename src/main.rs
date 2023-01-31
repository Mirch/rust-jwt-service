use rust_jwt_service::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
