use std::net::TcpListener;
use NewsLetterAPI::startup::run;
use NewsLetterAPI::configuration::get_configuration;
use NewsLetterAPI::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPool;
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let subscriber = get_subscriber("NewsLetterAPI".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if it cant read configuration file
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await;
    Ok(())

}