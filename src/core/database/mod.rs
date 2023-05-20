//! Database module
//!
//! This module contains the implementation of the Database connection
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

use dbzlib_rs::{
    model::character::Character,
    util::error::{Error, Result},
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct Database {
    pool: sqlx::Pool<Postgres>,
}

impl Database {
    /// Creates a new instance of Database and establishes a connection
    pub async fn new() -> Result<Self> {
        // Establishes a connection the local CharacterDB
        let pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect("postgresql://postgres@127.0.0.1/characterdb")
            .await
        {
            Ok(pool) => pool,
            Err(error) => return Err(Error::DatabaseConnection(error.to_string())),
        };

        Ok(Self { pool })
    }

    /// Returns a reference to the Pool instance
    pub fn pool(&self) -> &sqlx::Pool<Postgres> {
        &self.pool
    }
}

/// Enables interaction with the Character table
pub struct CharacterRepository<'a> {
    pool: &'a sqlx::Pool<Postgres>,
}

impl<'a> CharacterRepository<'a> {
    /// Creates an instance of CharacterRepository
    ///
    /// # Arguments
    /// * pool - the Database PostgreSQL connection pool
    pub fn new(pool: &'a sqlx::Pool<Postgres>) -> Self {
        Self { pool }
    }

    /// Adds a character to the repository
    ///
    /// # Arguments
    /// * character - the Character to add
    pub async fn add(&self, character: Character) -> Result<()> {
        let character = sqlx::query("INSERT INTO character(name, image_url) VALUES($1, $2)")
            .bind(character.name())
            .bind(character.image_url())
            .execute(self.pool)
            .await;

        println!("{:?}", character);
        Ok(())
    }

    /// Returns a character with same id as the one passed as parameter
    ///
    /// # Arguments:
    /// * id - the character id
    pub async fn get(&self, id: i64) -> Result<Character> {
        let character = sqlx::query_as::<_, Character>("SELECT * FROM character WHERE id = $1")
            .bind(id)
            .fetch_one(self.pool)
            .await;

        println!("{:?}", character);
        Ok(character.unwrap())
    }
}
