//! Error handling module
//!
//! This module contains implementations about error handling for Dragon Bot Z
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

use core::fmt;

// Custom Result type
type Result<T> = std::result::Result<T, Error>;

/// Not recoverable errors
pub enum Error {
    // Error thrown when the program is unable to connect to a Database
    DatabaseConnection(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // content to write
        let content = match &self {
            Error::DatabaseConnection(error) => format!("[Error][Database Connection] An error occured while trying to connect to Database: {error}"),
        };

        write!(f, "{}", content)
    }
}
