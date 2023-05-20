//! Route module
//!
//! This module contains the implementation of the Service's API routes
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

use actix_web::{get, post, web, HttpResponse, Responder};
use dbzlib_rs::{model::character::Character, util::error::Error};

use crate::core::database::{CharacterRepository, Database};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello and welcome to Dragon Bot Z's Character Service!")
}

#[post("/add")]
async fn add(character: web::Json<Character>) -> impl Responder {
    // connect database
    let database = Database::new().await;
    if let Err(error) = database {
        println!("{}", Error::DatabaseConnection(error.to_string()));
        return HttpResponse::InternalServerError()
            .body(format!("{}", Error::DatabaseConnection(error.to_string())));
    }
    let database = database.unwrap();

    if let Err(error) = CharacterRepository::new(database.pool())
        .add(character.into_inner())
        .await
    {
        return HttpResponse::InternalServerError().body("Unable to add character");
    }

    database.pool().close().await;
    HttpResponse::Ok().body("The character has been added!")
}
