// cargo run -p ex_lsp_03

// =========================
// Storage Backends - Problem
// =========================

use std::collections::HashMap;

// =========================
// Abstractions
// =========================

pub trait Storage {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: String, value: String);
    fn delete(&mut self, key: &str) -> bool;
}

// Simple Redis mock so the example compiles
pub struct RedisClient;

impl RedisClient {
    fn get(&self, _key: &str) -> Result<String, ()> {
        Err(())
    }
    fn set(&self, _key: &str, _value: &str) -> Result<(), ()> {
        Ok(())
    }
    fn del(&self, _key: &str) -> Result<(), ()> {
        Ok(())
    }
}

// =========================
// Concrete storages
// =========================

// In-memory backend
pub struct MemoryStorage {
    data: HashMap<String, String>,
}

impl MemoryStorage {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Storage for MemoryStorage {
    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }
}

// Redis backend
pub struct RedisStorage {
    client: RedisClient,
}

impl RedisStorage {
    fn new() -> Self {
        Self {
            client: RedisClient,
        }
    }
}

impl Storage for RedisStorage {
    fn get(&self, key: &str) -> Option<String> {
        self.client.get(key).ok()
    }

    fn set(&mut self, key: String, value: String) {
        self.client.set(&key, &value).ok();
    }

    fn delete(&mut self, key: &str) -> bool {
        self.client.del(key).is_ok()
    }
}

// BAD: File storage that violates LSP
pub struct FileStorage {
    base_path: String,
}

impl FileStorage {
    fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }
}

impl Storage for FileStorage {
    fn get(&self, key: &str) -> Option<String> {
        // Path traversal, filename length, permissions, etc.
        std::fs::read_to_string(format!("{}/{}", self.base_path, key)).ok()
    }

    fn set(&mut self, key: String, value: String) {
        // Fails silently if disk is full
        std::fs::write(format!("{}/{}", self.base_path, key), value).ok();
    }

    fn delete(&mut self, key: &str) -> bool {
        // Lies if file never existed
        std::fs::remove_file(format!("{}/{}", self.base_path, key)).is_ok()
    }
}

// =========================
// Usage
// =========================

// Generic function using the Storage trait
fn demo(storage: &mut dyn Storage) {
    storage.set("key".into(), "value".into());
    println!("Value = {:?}", storage.get("key"));
    println!("Deleted = {}", storage.delete("key"));
}

fn main() {
    let mut mem = MemoryStorage::new();
    let mut redis = RedisStorage::new();
    let mut file = FileStorage::new(".");

    demo(&mut mem);
    demo(&mut redis);
    demo(&mut file); // LSP violations hidden behind the trait
}
