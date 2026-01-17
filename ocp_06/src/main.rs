// cargo run -p ex_06_ocp
// ! DOES NOT COMPILE

// =========================
// Static Dispatch Based Solution - Txt Processor with Plugins
// =========================

// =========================
// Abstractions
// =========================

// Generic definition. A TxtProcessor is a vector of processing to be applied on text
// It knows nothing about the processing nor the text
pub struct TxtProcessor<T: Processing> {
    processings: Vec<T>,
}

impl<T: Processing> TxtProcessor<T> {
    pub fn new() -> Self {
        Self {
            processings: Vec::new(),
        }
    }

    pub fn register_processing(&mut self, tool: T) {
        self.processings.push(tool);
    }

    pub fn run(&mut self, context: &mut EditorContent) {
        for tool in &mut self.processings {
            println!("Running tool: {}", tool.name());
            tool.apply(context); // Direct call, no vtable
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

    // Tools must be of the same concrete type T
    processor.register_processing(LowerCase);
    processor.register_processing(SpellChecker);

    let mut ed_context = EditorContent {
        content: String::from("HELLO WORLD"),
    };

    processor.run(&mut ed_context);

    println!("--- FINAL CONTENT ---");
    println!("{}", ed_context.content);
}
