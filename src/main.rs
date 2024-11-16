use std::net::TcpListener;
use newsletterapi::startup::run;
use newsletterapi::configuration::get_configuration;
use newsletterapi::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPool;
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let subscriber = get_subscriber("NewsLetterAPI".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if it cant read configuration file
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await;
    Ok(())

}