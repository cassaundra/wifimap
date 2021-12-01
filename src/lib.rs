#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_with;

pub mod error;
pub use error::*;

pub mod db;
pub mod gps;
pub mod wifi;
