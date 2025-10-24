pub mod database;
pub mod models;
pub mod discovery;
pub mod validation;
pub mod errors;

// Re-export validation module
pub use validation::ValidationFramework;

pub use database::Database;
pub use errors::{ResearchError, Result};