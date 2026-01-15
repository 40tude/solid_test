// src/infrastructure/db.rs
// Infrastructure/DBA's responsibility

use std::fmt;

// Dummy database error type
#[derive(Debug)]
pub struct DbError;

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Database error")
    }
}

//Dummy database abstraction
pub struct Database;

impl Database {
    // Execute a fake SQL query
    pub fn execute(&self, query: &str, params: &[&dyn fmt::Debug]) -> Result<(), DbError> {
        println!("Executing SQL: {}", query);
        println!("With params: {:?}", params);
        Ok(())
    }
}
