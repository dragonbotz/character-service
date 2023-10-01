//! Entry point of Dragon Bot Z's Character Service.
//!
//! This Service enables the clients to have access to characters' information
//! and manipulate ther characters database.
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

mod core;

use crate::core::api::route;
use crate::core::database::Database;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() {
    let database = Database::new().await;
    if let Err(error) = database {
        panic!("{error}")
    }
    let database = database.unwrap();

    // Setup server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.pool().clone()))
            .service(route::root)
            .service(route::add)
            .service(route::get)
            .service(route::get_many)
    })
    .bind(("0.0.0.0", 8080));
    if let Err(error) = server {
        panic!("An error occured while binding server to ip adress and port: {error}")
    }

    // Runs the server
    let running_server = server.unwrap().run().await;
    if let Err(error) = running_server {
        panic!("An error occured while running the server: {error}")
    }
}
