//! Entry point of Dragon Bot Z's Character Service.
//!
//! This Service enables the clients to have access to characters' information
//! and manipulate ther characters database.
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

mod core;

use crate::core::api::route;

use std::process::exit;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() {
    let mut server = HttpServer::new(|| App::new().service(route::root));

    server = match server.bind(("127.0.0.1", 8080)) {
        Ok(server) => server,
        Err(error) => {
            println!("An error occured while binding ip adress and port to server: {error}");
            exit(1);
        }
    };

    println!("Character service's API running: http://127.0.0.1:8080/");

    if let Err(error) = server.run().await {
        println!("An error occured while running the HTTP server: {error}");
        exit(1);
    }
}
