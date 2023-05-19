//! Database module
//!
//! This module contains the implementation of the Database connection
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
use dbzlib_rs::util::error::{Error, Result};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

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
            Err(error) => return Err(Error::DatabaseConnection(error)),
        };

        Ok(Self { pool })
    }
}
