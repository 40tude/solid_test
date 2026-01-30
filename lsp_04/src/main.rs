// cargo run -p ex_lsp_04

// =========================
// Storage Backends - Fix
// =========================

use std::collections::HashMap;
use std::path::{Path, PathBuf};

// =========================
// Abstractions
// =========================

pub trait Storage {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: String, value: String);
    fn delete(&mut self, key: &str) -> bool;
}

// Simple Redis mock so the example compiles
// pub struct RedisClient;

// impl RedisClient {
//     fn get(&self, _key: &str) -> Result<String, ()> {
//         Err(())
//     }
//     fn set(&self, _key: &str, _value: &str) -> Result<(), ()> {
//         Ok(())
//     }
//     fn del(&self, _key: &str) -> Result<(), ()> {
//         Ok(())
//     }
// }

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
// pub struct RedisStorage {
//     client: RedisClient,
// }

// impl RedisStorage {
//     fn new() -> Self {
//         Self {
//             client: RedisClient,
//         }
//     }
// }

// impl Storage for RedisStorage {
//     fn get(&self, key: &str) -> Option<String> {
//         self.client.get(key).ok()
//     }

//     fn set(&mut self, key: String, value: String) {
//         self.client.set(&key, &value).ok();
//     }

//     fn delete(&mut self, key: &str) -> bool {
//         self.client.del(key).is_ok()
//     }
// }

// =========================
// FIXED: LSP-compliant FileStorage
// =========================

pub struct FileStorage {
    base_path: String,
}

impl FileStorage {
    fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    // Prevent path traversal and invalid filenames
    fn validate_key(&self, key: &str) -> bool {
        !key.contains("..") && !key.contains('/') && !key.contains('\\') && key.len() <= 255
    }

    fn key_to_path(&self, key: &str) -> PathBuf {
        Path::new(&self.base_path).join(key)
    }
}

impl Storage for FileStorage {
    fn get(&self, key: &str) -> Option<String> {
        // Invalid keys behave like "not found"
        if !self.validate_key(key) {
            return None;
        }

        let path = self.key_to_path(key);
        // IO errors are mapped to None, just like missing keys
        std::fs::read_to_string(path).ok()
    }

    fn set(&mut self, key: String, value: String) {
        if !self.validate_key(&key) {
            return;
        }

        let path = self.key_to_path(&key);

        // Ensure failures are no longer silent
        if let Err(e) = std::fs::write(path, value) {
            eprintln!("FileStorage set failed: {}", e);
        }
    }

    fn delete(&mut self, key: &str) -> bool {
        if !self.validate_key(key) {
            return false;
        }

        let path = self.key_to_path(key);

        match std::fs::remove_file(path) {
            Ok(()) => true, // File really existed and was deleted
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => false,
            Err(e) => {
                eprintln!("FileStorage delete failed: {}", e);
                false
            }
        }
    }
}

// =========================
// Usage
// =========================

fn demo(storage: &mut dyn Storage) {
    storage.set("key".into(), "value".into());
    println!("Value = {:?}", storage.get("key"));
    println!("Deleted = {}", storage.delete("key"));
}

fn main() {
    let mut mem = MemoryStorage::new();
    // let mut redis = RedisStorage::new();
    let mut file = FileStorage::new(".");

    demo(&mut mem);
    // demo(&mut redis);
    demo(&mut file);
}
