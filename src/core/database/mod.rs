//! Database module
//!
//! This module contains the implementation of the Database connection
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

use dbzlib_rs::{
    model::character::{Character, CharacterBuilder},
    util::error::{ErrResult, Error},
    util::exception::{ExcResult, Exception},
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};

#[derive(Clone)]
pub struct Database {
    pool: sqlx::Pool<Postgres>,
}

impl Database {
    /// Creates a new instance of Database and establishes a connection
    pub async fn new() -> ErrResult<Self> {
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
    pub async fn add(&self, character: Character) -> ExcResult<()> {
        let result = sqlx::query("INSERT INTO character(name, image_url) VALUES($1, $2)")
            .bind(character.name())
            .bind(character.image_url())
            .execute(self.pool)
            .await;

        // unable to add a new character
        if let Err(error) = result {
            return Err(Exception::InsertNewCharacter(error.to_string()));
        }

        Ok(())
    }

    /// Returns a character with same id as the one passed as parameter
    ///
    /// # Arguments:
    /// * id - the character id
    pub async fn get(&self, id: i64) -> ExcResult<Character> {
        let character = sqlx::query_as::<_, Character>("SELECT * FROM character WHERE id = $1")
            .bind(id)
            .fetch_one(self.pool)
            .await;

        // unable to retrieve the character
        if let Err(error) = character {
            return Err(Exception::RetrieveCharacter(error.to_string()));
        }

        Ok(character.unwrap())
    }

    /// Returns a collection of character according to ids passed as parameter
    ///
    /// # Arguments
    /// * ids - the collection of ids to retrieve characters from
    pub async fn get_many(&self, ids: Vec<i64>) -> ExcResult<Vec<Character>> {
        // creates a comma separated list of ids as String
        let ids = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let result = sqlx::query(format!("SELECT * FROM character WHERE id IN ({})", ids).as_str())
            .fetch_all(self.pool)
            .await;

        // unable to retrieve all the characters
        if let Err(error) = result {
            return Err(Exception::RetrieveMultipleCharacters(error.to_string()));
        }
        let characters = result.unwrap();

        // for each entries, retrieve the character
        // and add it to the character collection
        let mut characters_collection = Vec::<Character>::new();
        for row in characters {
            let character = CharacterBuilder::new()
                .id(row.get::<i64, _>("id"))
                .name(row.get::<String, _>("name"))
                .image_url(row.get::<String, _>("image_url"))
                .build();

            characters_collection.push(character);
        }

        Ok(characters_collection)
    }
}
