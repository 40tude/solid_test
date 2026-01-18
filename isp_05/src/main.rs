// cargo run -p ex_05_isp

// =========================
// Combine Traits - Use Trait Bounds
// =========================

// =========================
// Abstractions
// =========================
mod traits {
    // Allows reading data
    pub trait Queryable {
        fn query(&self, sql: &str) -> Result<Vec<String>, String>;
    }

    // Allows executing commands
    pub trait Executable {
        fn execute(&mut self, command: &str) -> Result<(), String>;
    }
}

// =========================
// Concrete MemoryStorage
// =========================

mod storage {
    use super::traits::{Executable, Queryable};

    // A simple in-memory storage
    pub struct MemoryStorage {
        pub data: Vec<String>,
    }

    impl Queryable for MemoryStorage {
        fn query(&self, _sql: &str) -> Result<Vec<String>, String> {
            Ok(self.data.clone())
        }
    }

    impl Executable for MemoryStorage {
        fn execute(&mut self, command: &str) -> Result<(), String> {
            println!("Executing: {}", command);
            self.data.push(command.to_string());
            Ok(())
        }
    }
}

// =========================
// Usage
// =========================

use storage::MemoryStorage;
use traits::{Executable, Queryable};

// Uses generic trait bounds
fn replicate<C>(source: &mut C, dest: &mut C) -> Result<(), String>
where
    C: Queryable + Executable,
{
    let data = source.query("SELECT * FROM table")?;

    for row in data {
        let cmd = format!("INSERT INTO table VALUES ({})", row);
        dest.execute(&cmd)?;
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let mut source = MemoryStorage {
        data: vec!["Alice".into(), "Bob".into()],
    };

    let mut dest = MemoryStorage { data: Vec::new() };

    replicate(&mut source, &mut dest)?;

    println!("Destination data: {:?}", dest.data);
    Ok(())
}
