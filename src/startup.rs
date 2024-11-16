use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {

    // wrap the pool using web::Data which boils down to an arc smart pointer
    let db_pool = web::Data::new(db_pool);
    // capture `connection` from the surr
    let server = HttpServer::new(move || {
        App::new()
            // Added tracing Logger to have a consistent request_id for each endpoint
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // get a pointer copy and attach it to application state.
            .app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}