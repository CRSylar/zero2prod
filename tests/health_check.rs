use std::net::TcpListener;
use tokio::spawn;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind at random port.");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = spawn(server);

    return format!("http://127.0.0.1:{}", port);
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    let res = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

#[tokio::test]
async fn subscribe_ok() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=cristiano%20romaldetti&email=cristianoromaldetti%40gmail.com";

    let res = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute Request.");

    assert_eq!(200, res.status().as_u16());
}

#[tokio::test]
async fn subscribe_fail() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = [
        ("name=cristiano%20romaldetti", "Missing the Email field"),
        (
            "email=cristianoromaldetti%40gmail.com",
            "Missing the name field",
        ),
        ("", "Missing both name & Email fields"),
    ];

    for (body, message) in test_cases {
        let res = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute Request.");

        assert_eq!(
            400,
            res.status().as_u16(),
            "The Api did not fail with 400 when the Payload was {}.",
            message
        );
    }
}
