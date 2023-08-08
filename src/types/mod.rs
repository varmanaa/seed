use crate::utility::error::Error;

pub mod cache;
pub mod context;
pub mod database;
pub mod interaction;
pub mod surface;

pub type Result<T> = std::result::Result<T, Error>;
