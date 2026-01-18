// cargo run -p ex_03_isp

// =========================
// God Trait Fix
// =========================

// =========================
// Abstractions
// =========================

mod domain {
    #[derive(Debug, Clone)]
    pub struct Metadata {
        pub title: String,
    }
}

mod traits {
    use super::domain::Metadata;

    // Core reading operations
    pub trait Readable {
        fn get_content(&self) -> &str;
        fn get_metadata(&self) -> &Metadata;
    }
}

// =========================
// Concrete read-only archiver
// =========================

mod archive {
    use super::domain::Metadata;
    use super::traits::Readable;

    // A read-only archive document
    pub struct ArchiveDocument {
        content: String,
        metadata: Metadata,
    }

    impl ArchiveDocument {
        pub fn new(content: String, title: String) -> Self {
            Self {
                content,
                metadata: Metadata { title },
            }
        }
    }

    impl Readable for ArchiveDocument {
        fn get_content(&self) -> &str {
            &self.content
        }

        fn get_metadata(&self) -> &Metadata {
            &self.metadata
        }
    }
}

// =========================
// Usage
// =========================

use archive::ArchiveDocument;
use traits::Readable;

fn main() {
    let archive = ArchiveDocument::new(
        "This is a historical document.".to_string(),
        "Company Archive 1998".to_string(),
    );

    println!("Title: {}", archive.get_metadata().title);
    println!("Content: {}", archive.get_content());
}
