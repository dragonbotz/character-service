//! Entry point of Dragon Bot Z's Character Service.
//!
//! This Service enables the clients to have access to characters' information
//! and manipulate ther characters database.
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

mod core;

use crate::core::api::route;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() {
    // Setup server
    let server = HttpServer::new(|| App::new().service(route::root).service(route::add))
        .bind(("127.0.0.1", 8080));
    if let Err(error) = server {
        panic!("An error occured while binding server to ip adress and port: {error}")
    }

    // Runs the server
    let running_server = server.unwrap().run().await;
    if let Err(error) = running_server {
        panic!("An error occured while running the server: {error}")
    }
}
