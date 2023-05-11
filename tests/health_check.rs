use sqlx::{Error, Executor, PgPool};
use std::net::TcpListener;
use tokio::spawn;
use uuid::Uuid;
use zero2prod::configuration::{get_config, DatabaseSettings};

pub struct TestApp {
    pub address: String,
    pub db_name: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind at random port.");
    let port = listener.local_addr().unwrap().port();

    let address = format!("http://127.0.0.1:{}", port);
    let mut config = get_config().expect("Failed to load configuration file !");
    config.database.db_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&config.database).await;

    let server =
        zero2prod::startup::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = spawn(server);

    TestApp {
        address,
        db_name: config.database.db_name,
        db_pool: connection_pool,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let connection = PgPool::connect(&config.connection_string_no_db())
        .await
        .expect("Failed to connect to Postgres DB");

    connection
        .execute(format!(r#"CREATE DATABASE"{}";"#, config.db_name).as_str())
        .await
        .expect("Failed to create new Database");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Database");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the Database");

    connection_pool
}

async fn cleanup_database(db_name: &str) -> Result<(), Error> {
    let config = get_config().expect("Failed to load configuration file !");

    let connection = PgPool::connect(&config.database.connection_string_no_db())
        .await
        .expect("Failed to connect to Postgres DB");

    let q = format!(r#"DROP DATABASE IF EXISTS "{}""#, db_name);

    sqlx::query(&q).execute(&connection).await?;

    Ok(())
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let TestApp {
        address,
        db_name,
        db_pool,
    } = spawn_app().await;
    let client = reqwest::Client::new();

    let res = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());

    db_pool.close().await;
    cleanup_database(&db_name)
        .await
        .expect(&format!("Cleaning failure on db {}", db_name));
}

#[tokio::test]
async fn subscribe_ok() {
    let TestApp {
        address,
        db_pool,
        db_name,
    } = spawn_app().await;

    let client = reqwest::Client::new();
    let body = "name=cristiano%20romaldetti&email=cristianoromaldetti%40gmail.com";

    let res = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute Request.");

    assert_eq!(200, res.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&db_pool)
        .await
        .expect("Failed to fetch saved records");

    assert_eq!(saved.email, "cristianoromaldetti@gmail.com");
    assert_eq!(saved.name, "cristiano romaldetti");

    db_pool.close().await;
    cleanup_database(&db_name)
        .await
        .expect(&format!("Cleaning failure on db {}", db_name));
}

#[tokio::test]
async fn subscribe_fail() {
    let TestApp {
        address,
        db_name,
        db_pool,
    } = spawn_app().await;
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
            .post(&format!("{}/subscriptions", &address))
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

    db_pool.close().await;
    cleanup_database(&db_name)
        .await
        .expect(&format!("Cleaning failure on db {}", db_name));
}
