// cargo run -p ex_04_ocp

// =========================
// Naïve Solution - Txt Processor with Plugins
// =========================

// =========================
// Abstractions
// =========================

// A TxtProcessor knows nothing about the processing nor the text
pub struct TxtProcessor;

impl TxtProcessor {
    pub fn run<P1: Processing, P2: Processing>(
        &self,
        processing1: &P1,
        processing2: &P2,
        content: &mut EditorContent,
    ) {
        processing1.apply(content);
        processing2.apply(content);
    }
}

// Here the content of the Editor is just a String
pub struct EditorContent {
    pub content: String,
}

// If a type wants to have the Processing trait it must implement the .apply() method
pub trait Processing {
    fn apply(&self, context: &mut EditorContent);
}

// =========================
// Concrete processing
// =========================

// Lowercase processing
pub struct LowerCase;

impl Processing for LowerCase {
    fn apply(&self, context: &mut EditorContent) {
        context.content = context.content.to_lowercase();
        context.content.push_str("\n[LowerCase OK]");
    }
}

// SpellChecker processing
pub struct SpellChecker;

impl Processing for SpellChecker {
    fn apply(&self, context: &mut EditorContent) {
        context.content.push_str("\n[SpellChecker OK]");
    }
}

// =========================
// Usage
// =========================

fn main() {
    let processor = TxtProcessor;

    let lowercase = LowerCase;
    let spell_checker = SpellChecker;

    let mut context = EditorContent {
        content: String::from("HELLO WORLD"),
    };

    processor.run(&lowercase, &spell_checker, &mut context);

    println!("--- FINAL CONTENT ---");
    println!("{}", context.content);
}
