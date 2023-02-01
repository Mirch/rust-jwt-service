use std::net::TcpListener;

pub async fn create_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address.");
    let address = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());

    let server = rust_jwt_service::startup::run(listener).expect("Failed to create server.");
    let _ = tokio::spawn(server);

    address
}