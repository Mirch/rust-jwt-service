mod test_app;

use claim::assert_ok;
use reqwest::Client;
use rust_jwt_service::routes::UserAuthResponse;
use serde_json::json;
use test_app::create_app;

#[tokio::test]
async fn login_with_existing_credentials_returns_token() {
    let app_address = create_app().await;
    let client = Client::new();

    let response = client
        .post(format!("{}/login", app_address))
        .header("Content-type", "application/json")
        .body(json!({"username": "admin", "password": "admin"}).to_string())
        .send()
        .await
        .expect("Failed to send login request.");

    assert!(response.status().is_success());
    let token_response = response.json::<UserAuthResponse>().await;
    assert_ok!(token_response);
}

#[tokio::test]
async fn login_with_wrong_password_returns_bad_request() {
    let app_address = create_app().await;
    let client = Client::new();

    let response = client
        .post(format!("{}/login", app_address))
        .header("Content-type", "application/json")
        .body(json!({"username": "admin", "password": "not-the-right-password"}).to_string())
        .send()
        .await
        .expect("Failed to send login request.");

    let status = response.status().as_u16();
    assert_eq!(400, status);
}

#[tokio::test]
async fn login_with_nonexistent_username_returns_not_found() {
    let app_address = create_app().await;
    let client = Client::new();

    let response = client
        .post(format!("{}/login", app_address))
        .header("Content-type", "application/json")
        .body(json!({"username": "not-a-username", "password": "password"}).to_string())
        .send()
        .await
        .expect("Failed to send login request.");

    let status = response.status().as_u16();
    assert_eq!(404, status);
}

#[tokio::test]
async fn register_with_new_credentials_returns_token() {
    let app_address = create_app().await;
    let client = Client::new();

    let response = client
        .post(format!("{}/register", app_address))
        .header("Content-type", "application/json")
        .body(json!({"username": "new-username", "password": "password"}).to_string())
        .send()
        .await
        .expect("Failed to send login request.");

    assert!(response.status().is_success());
    let token_response = response.json::<UserAuthResponse>().await;
    assert_ok!(token_response);
}

#[tokio::test]
async fn register_with_existing_username_returns_bad_request() {
    let app_address = create_app().await;
    let client = Client::new();

    let response = client
        .post(format!("{}/register", app_address))
        .header("Content-type", "application/json")
        .body(json!({"username": "admin", "password": "password"}).to_string())
        .send()
        .await
        .expect("Failed to send login request.");

    let status = response.status().as_u16();
    assert_eq!(400, status);
}

#[tokio::test]
async fn get_secret_info_with_token_returns_ok() {
    let app_address = create_app().await;
    let client = Client::new();

    let token = client
        .post(format!("{}/login", app_address))
        .header("Content-type", "application/json")
        .body(json!({"username": "admin", "password": "admin"}).to_string())
        .send()
        .await
        .expect("Failed to send login request.")
        .json::<UserAuthResponse>()
        .await
        .expect("Failed to deserialize login response.")
        .token;

    let response = client
        .get(format!("{}/secret", app_address))
        .bearer_auth(token)
        .send()
        .await
        .expect("Failed to send login request.");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn get_secret_info_without_token_returns_unauthorized() {
    let app_address = create_app().await;
    let client = Client::new();

    let response = client
        .get(format!("{}/secret", app_address))
        .send()
        .await
        .expect("Failed to send login request.");

    println!("{:?}", response);

    let status = response.status().as_u16();
    assert_eq!(401, status);
}
