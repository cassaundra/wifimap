use thiserror::Error;

use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0:?}")]
    IO(#[from] io::Error),
    #[error("CSV error: {0:?}")]
    CSV(#[from] csv::Error),
    #[error("Database error: {0:?}")]
    Database(#[from] diesel::result::Error),
}
