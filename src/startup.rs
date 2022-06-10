use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

use crate::routes::{health_check, subsribes};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/subscriptions", web::post().to(subsribes))
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
