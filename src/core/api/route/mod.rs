//! Route module
//!
//! This module contains the implementation of the Service's API routes
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

use actix_web::{get, post, web, HttpResponse, Responder};
use dbzlib_rs::{model::character::Character, util::error::Error};
use sqlx::{Pool, Postgres};

use crate::core::database::{CharacterRepository, Database};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello and welcome to Dragon Bot Z's Character Service!")
}

#[post("/add")]
async fn add(
    database_pool: web::Data<Pool<Postgres>>,
    character: web::Json<Character>,
) -> impl Responder {
    if let Err(error) = CharacterRepository::new(&database_pool)
        .add(character.into_inner())
        .await
    {
        return HttpResponse::InternalServerError().body("Unable to add character");
    }

    HttpResponse::Ok().body("The character has been added!")
}

#[get("/get/{id}")]
async fn get(database_pool: web::Data<Pool<Postgres>>, id: web::Path<i64>) -> impl Responder {
    CharacterRepository::new(&database_pool)
        .get(id.into_inner())
        .await;
    HttpResponse::Ok()
}
