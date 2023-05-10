use tokio::spawn;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();

    let client = reqwest::Client::new();

    let res = client
        .get("http://127.0.0.1:3000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = spawn(server);
}
