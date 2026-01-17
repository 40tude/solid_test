// cargo run -p ex_05_ocp

// =========================
// Dynamic Dispatch Based Solution - Txt Processor with Plugins
// =========================

// =========================
// Abstractions
// =========================

// A TxtProcessor is a vector of processing to be applied on text
// It knows nothing about the processing nor the text
pub struct TxtProcessor {
    processings: Vec<Box<dyn Processing>>,
}

impl TxtProcessor {
    pub fn new() -> Self {
        Self {
            processings: Vec::new(),
        }
    }

    pub fn register_processing(&mut self, processing: Box<dyn Processing>) {
        self.processings.push(processing);
    }

    pub fn run(&mut self, content: &mut EditorContent) {
        for processing in &mut self.processings {
            println!("Running processing: {}", processing.name());
            processing.apply(content); // Apply the processing to the shared content
        }
    }
}

// Here the content of the TxtProcessor is just a String
pub struct EditorContent {
    pub content: String,
}

// If a type wants to have the Processing trait it must implement the 2 methods below
pub trait Processing {
    fn name(&self) -> &str;
    fn apply(&mut self, context: &mut EditorContent);
}

// =========================
// Concrete processing
// =========================

// Lowercase processing
pub struct LowerCase;

impl Processing for LowerCase {
    fn name(&self) -> &str {
        "LowerCase"
    }

    fn apply(&mut self, context: &mut EditorContent) {
        context.content = context.content.to_lowercase();
        context.content.push_str("\n[LowerCase OK]");
    }
}

// SpellChecker processing
pub struct SpellChecker;

impl Processing for SpellChecker {
    fn name(&self) -> &str {
        "SpellChecker"
    }

    fn apply(&mut self, context: &mut EditorContent) {
        // Fake spell checker
        context.content.push_str("\n[SpellChecker OK]");
    }
}

// =========================
// Usage
// =========================

fn main() {
    let mut processor = TxtProcessor::new();

    processor.register_processing(Box::new(LowerCase));
    processor.register_processing(Box::new(SpellChecker));

    let mut ed_context = EditorContent {
        content: String::from("HELLO WORLD"),
    };

    processor.run(&mut ed_context);

    println!("--- FINAL CONTENT ---");
    println!("{}", ed_context.content);
}
