use std::net::TcpListener;
use newsletterapi::startup::run;
use newsletterapi::configuration::get_configuration;
use newsletterapi::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let subscriber = get_subscriber("newsletterapi".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if it cant read configuration file
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .connect_lazy_with(configuration.database.with_db());

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await;
    Ok(())

}