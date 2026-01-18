// cargo run -p ex_01_isp

// =========================
// God Trait Problem
// =========================

// =========================
// Abstractions
// =========================

mod domain {
    #[derive(Debug, Clone)]
    pub struct Metadata {
        pub title: String,
    }

    #[derive(Debug, Clone)]
    pub struct Version {
        pub id: u32,
    }

    #[derive(Debug)]
    pub struct User {
        pub name: String,
    }

    #[derive(Debug)]
    pub struct Comment {
        pub text: String,
    }

    #[derive(Debug)]
    pub enum Permission {
        Read,
        Write,
    }
}

mod document {
    use super::domain::*;

    // The "God Trait"
    pub trait Document {
        // Reading
        fn get_content(&self) -> &str;
        fn get_metadata(&self) -> &Metadata;
        fn search(&self, query: &str) -> Vec<usize>;

        // Writing
        fn set_content(&mut self, content: String);
        fn append(&mut self, text: &str);
        fn insert(&mut self, pos: usize, text: &str);

        // Formatting
        fn to_html(&self) -> String;
        fn to_markdown(&self) -> String;
        fn to_pdf(&self) -> Vec<u8>;

        // Versioning
        fn save_version(&mut self) -> Version;
        fn list_versions(&self) -> Vec<Version>;
        fn restore_version(&mut self, version: &Version);

        // Permissions
        fn can_read(&self, user: &User) -> bool;
        fn can_write(&self, user: &User) -> bool;
        fn share_with(&mut self, user: &User, permission: Permission);

        // Collaboration
        fn add_comment(&mut self, comment: Comment);
        fn list_comments(&self) -> &[Comment];
        fn notify_watchers(&self);
    }
}

// =========================
// Concrete read-only viewer
// =========================

mod viewer {
    use super::document::Document;
    use super::domain::*;

    pub struct ReadOnlyViewer {
        content: String,
        metadata: Metadata,
        comments: Vec<Comment>,
    }

    impl ReadOnlyViewer {
        pub fn new(content: String, title: String) -> Self {
            Self {
                content,
                metadata: Metadata { title },
                comments: Vec::new(),
            }
        }
    }

    // Forced to implement EVERYTHING, even useless methods
    impl Document for ReadOnlyViewer {
        // Reading (the only useful part)
        fn get_content(&self) -> &str {
            &self.content
        }

        fn get_metadata(&self) -> &Metadata {
            &self.metadata
        }

        fn search(&self, query: &str) -> Vec<usize> {
            self.content.match_indices(query).map(|(i, _)| i).collect()
        }

        // Writing (not supported)
        fn set_content(&mut self, _content: String) {
            panic!("Read-only viewer cannot modify content");
        }

        fn append(&mut self, _text: &str) {
            panic!("Read-only viewer cannot append text");
        }

        fn insert(&mut self, _pos: usize, _text: &str) {
            panic!("Read-only viewer cannot insert text");
        }

        // Formatting (not supported)
        fn to_html(&self) -> String {
            panic!("Read-only viewer cannot export to HTML");
        }

        fn to_markdown(&self) -> String {
            panic!("Read-only viewer cannot export to Markdown");
        }

        fn to_pdf(&self) -> Vec<u8> {
            panic!("Read-only viewer cannot export to PDF");
        }

        // Versioning (not supported)
        fn save_version(&mut self) -> Version {
            panic!("Read-only viewer does not support versioning");
        }

        fn list_versions(&self) -> Vec<Version> {
            Vec::new()
        }

        fn restore_version(&mut self, _version: &Version) {
            panic!("Read-only viewer cannot restore versions");
        }

        // Permissions (hardcoded)
        fn can_read(&self, _user: &User) -> bool {
            true
        }

        fn can_write(&self, _user: &User) -> bool {
            false
        }

        fn share_with(&mut self, _user: &User, _permission: Permission) {
            panic!("Read-only viewer cannot share documents");
        }

        // Collaboration (mostly useless)
        fn add_comment(&mut self, comment: Comment) {
            self.comments.push(comment);
        }

        fn list_comments(&self) -> &[Comment] {
            &self.comments
        }

        fn notify_watchers(&self) {
            // No-op for a simple viewer
        }
    }
}

// =========================
// Usage
// =========================

use document::Document;
use viewer::ReadOnlyViewer;

fn main() {
    let viewer = ReadOnlyViewer::new("Hello SOLID world!".to_string(), "ISP Example".to_string());

    println!("Title: {}", viewer.get_metadata().title);
    println!("Content: {}", viewer.get_content());
    println!("Search 'SOLID': {:?}", viewer.search("SOLID"));
}
