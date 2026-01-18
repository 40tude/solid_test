// cargo run -p ex_02_isp

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

    // Full-text search
    pub trait Searchable {
        fn search(&self, query: &str) -> Vec<usize>;
    }
}

// =========================
// Concrete read-only viewer
// =========================

mod viewer {
    use super::domain::Metadata;
    use super::traits::{Readable, Searchable};

    // A simple read-only viewer
    pub struct ReadOnlyViewer {
        content: String,
        metadata: Metadata,
    }

    impl ReadOnlyViewer {
        pub fn new(content: String, title: String) -> Self {
            Self {
                content,
                metadata: Metadata { title },
            }
        }
    }

    impl Readable for ReadOnlyViewer {
        fn get_content(&self) -> &str {
            &self.content
        }

        fn get_metadata(&self) -> &Metadata {
            &self.metadata
        }
    }

    impl Searchable for ReadOnlyViewer {
        fn search(&self, query: &str) -> Vec<usize> {
            self.content.match_indices(query).map(|(i, _)| i).collect()
        }
    }
}

// =========================
// Usage
// =========================

use traits::{Readable, Searchable};
use viewer::ReadOnlyViewer;

fn main() {
    let viewer = ReadOnlyViewer::new("Hello SOLID world!".to_string(), "ISP Example".to_string());

    println!("Title: {}", viewer.get_metadata().title);
    println!("Content: {}", viewer.get_content());
    println!("Search 'SOLID': {:?}", viewer.search("SOLID"));
}
