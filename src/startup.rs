use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {

    // wrap the connection to a smart pointer
    let connection = web::Data::new(connection);
    // capture `connection` from the surr
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // get a pointer copy and attach it to application state.
            .app_data(connection)
    })
        .listen(listener)?
        .run();
    Ok(server)
}