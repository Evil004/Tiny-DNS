pub mod deserialize;
pub mod serialize;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
