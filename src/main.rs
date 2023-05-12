use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_config;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let sub = get_subscriber("Zero2Prod".into(), "info".into(), std::io::stdout);

    init_subscriber(sub);

    let config = get_config().expect("Failed to load configuration file !");
    let address = format!("127.0.0.1:{}", config.application_port);

    let connection_pool = PgPool::connect(&config.database.connection_string().expose_secret())
        .await
        .expect("Failed to Connect to Postgres DB");
    let listener = TcpListener::bind(address).expect(&format!(
        "Failed to bind to port {}.",
        config.application_port
    ));
    run(listener, connection_pool)?.await?;
    Ok(())
}
