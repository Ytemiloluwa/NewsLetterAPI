use std::net::TcpListener;
use NewsLetterAPI::startup::run;
use NewsLetterAPI::configuration::get_configuration;
use sqlx::PgPool;
use env_logger::Env;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter:: { BunyanFormattingLayer, JsonStorageLayer };
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info")); let formatting_layer = BunyanFormattingLayer::new(
        "NewsLetterAPI".into(),
        std::io::stdout

    );
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");

    // Panic if it cant read configuration file
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await

}