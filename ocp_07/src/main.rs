// cargo run -p ex_07_ocp

// =========================
// Static Dispatch Based Solution - Txt Processor with Plugins
// =========================

// =========================
// Abstractions
// =========================

// A TxtProcessor is a toolchain of processing to be applied on text
// It knows nothing about the processing nor the text
pub struct TxTProcessor<T> {
    processings: T,
}

impl<T: ToolChain> TxTProcessor<T> {
    pub fn new(tools: T) -> Self {
        Self { processings: tools }
    }

    pub fn run(&mut self, context: &mut EditorContent) {
        self.processings.apply(context);
    }
}

// Apply a processing or a chain of processings
pub trait ToolChain {
    fn apply(&mut self, context: &mut EditorContent);
}

// Implementation for a unique processing
// A single Processing is also a valid ToolChain
// If we don't have this implementation there is no way to implement the recursive
impl<T: Processing> ToolChain for T {
    fn apply(&mut self, context: &mut EditorContent) {
        // self.apply(context); // ! CANNOT work: .apply() calls .apply()
        Processing::apply(self, context);
    }
}

// Recursive implementation for a tuple
// A tuple (Head, Tail) is a ToolChain if:
//      Head is a Processing
//      Tail is already a ToolChain
impl<Head, Tail> ToolChain for (Head, Tail)
where
    Head: Processing,
    Tail: ToolChain,
{
    fn apply(&mut self, context: &mut EditorContent) {
        self.0.apply(context);
        self.1.apply(context);
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
        "Git Integration"
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
    let mut processor = TxTProcessor::new((LowerCase, SpellChecker));

    // At this point the chain is complete:
    // LowerCase implements Tool
    // therefore LowerCase implements ToolChain
    // therefore (SpellCheck, LowerCase) implements ToolChain
    // therefore Editor<(SpellCheck, LowerCase)> is valid

    let mut ed_context = EditorContent {
        content: String::from("HELLO WORLD"),
    };

    processor.run(&mut ed_context);

    println!("--- FINAL CONTENT ---");
    println!("{}", ed_context.content);
}
