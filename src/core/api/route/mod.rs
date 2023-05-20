//! Route module
//!
//! This module contains the implementation of the Service's API routes
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

use actix_web::{get, post, web, HttpResponse, Responder};
use dbzlib_rs::model::character::Character;
use sqlx::{Pool, Postgres};

use crate::core::database::{CharacterRepository, Database};

/// Returns an hello world message
#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello and welcome to Dragon Bot Z's Character Service!")
}

/// Adds a character to the database
///
/// # Payload configuration
/// * Content-Type: application/json
/// * payload: JSON Character
#[post("/add")]
async fn add(
    database_pool: web::Data<Pool<Postgres>>,
    character: web::Json<Character>,
) -> impl Responder {
    if let Err(error) = CharacterRepository::new(&database_pool)
        .add(character.into_inner())
        .await
    {
        println!("{error}");
        return HttpResponse::BadRequest().body(format!("{error}"));
    }

    HttpResponse::Ok().body("The character has been added!")
}

/// Returns a JSON Character according to the id passed as request path
#[get("/get/{id}")]
async fn get(database_pool: web::Data<Pool<Postgres>>, id: web::Path<i64>) -> impl Responder {
    let character = CharacterRepository::new(&database_pool)
        .get(id.into_inner())
        .await;

    if let Err(error) = character {
        println!("{error}");
        return HttpResponse::NotFound().body(format!("{error}"));
    }
    let character = character.unwrap();

    HttpResponse::Ok().json(character)
}

/// Returns a JSON collection of Characters
///
/// # Payload configuration
/// * Content-Type: application/json
/// * payload: JSON [i64]
#[get("/get-many")]
async fn get_many(
    database_pool: web::Data<Pool<Postgres>>,
    ids: web::Json<Vec<i64>>,
) -> impl Responder {
    let ids = ids.into_inner();
    let characters = CharacterRepository::new(&database_pool).get_many(ids).await;

    if let Err(error) = characters {
        println!("{error}");
        return HttpResponse::NotFound().body(format!("{error}"));
    }
    let characters = characters.unwrap();

    HttpResponse::Ok().json(characters)
}
