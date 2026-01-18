// cargo run -p ex_04_isp

// =========================
// Combine Traits - Require multiple traits
// =========================

// =========================
// Abstractions
// =========================

mod traits {
    // Allows querying data
    pub trait Queryable {
        fn query(&self, sql: &str) -> Result<Vec<String>, String>;
    }

    // Allows transaction handling
    pub trait Transactional {
        fn begin_transaction(&mut self) -> Result<Transaction, String>;
    }

    pub struct Transaction;

    impl Transaction {
        pub fn commit(self) -> Result<(), String> {
            println!("Transaction committed");
            Ok(())
        }
    }
}

// =========================
// Concrete database
// =========================

mod database {
    use super::traits::{Queryable, Transaction, Transactional};

    // A database connection supporting both querying and transactions
    pub struct DatabaseConnection;

    impl Queryable for DatabaseConnection {
        fn query(&self, sql: &str) -> Result<Vec<String>, String> {
            println!("Running query: {}", sql);
            Ok(vec!["row1".to_string(), "row2".to_string()])
        }
    }

    impl Transactional for DatabaseConnection {
        fn begin_transaction(&mut self) -> Result<Transaction, String> {
            println!("Transaction started");
            Ok(Transaction)
        }
    }
}

// =========================
// Usage
// =========================

use database::DatabaseConnection;
use traits::{Queryable, Transactional};

// Requires multiple traits
fn backup_data(conn: &mut (impl Queryable + Transactional)) -> Result<(), String> {
    let tx = conn.begin_transaction()?;
    let data = conn.query("SELECT * FROM important_table")?;

    for row in data {
        println!("Backing up: {}", row);
    }

    tx.commit()
}

fn main() -> Result<(), String> {
    let mut db = DatabaseConnection;
    backup_data(&mut db)?;
    Ok(())
}
