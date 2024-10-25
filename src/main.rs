use std::net::TcpListener;
use NewsLetterAPI::startup::run;
use NewsLetterAPI::configuration::get_configuration;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if it cant read configuration file
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await

}