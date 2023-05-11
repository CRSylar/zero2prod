use std::fmt::format;
use std::net::TcpListener;
use zero2prod::configuration::get_config;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_config().expect("Failed to load configuration file !");
    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = TcpListener::bind(address).expect(&format!(
        "Failed to bind to port {}.",
        config.application_port
    ));
    run(listener)?.await
}
