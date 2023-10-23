use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgConnection;
use crate::routes::{health_check, subscribe};


pub fn run(listener: TcpListener, connection: PgConnection23) -> Result<Server, std::io::Error> {
    let server = HttpServer::new( || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            app.dataconec
    })
    .listen(listener)?
    .run();

    Ok(server)
}