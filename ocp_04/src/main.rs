// cargo run -p ex_04_ocp

// =========================
// Static dispatch example
// =========================

// =========================
// Abstractions
// =========================

// Editor with static dispatch
pub struct Editor;

impl Editor {
    pub fn run<T1: Tool, T2: Tool>(&self, tool1: &T1, tool2: &T2, context: &mut EditorContent) {
        tool1.apply(context);
        tool2.apply(context);
    }
}

// Here the content of the Editor is just a String
pub struct EditorContent {
    pub content: String,
}

// If a type wants to have the Tool trait it must implement the apply method
pub trait Tool {
    fn apply(&self, context: &mut EditorContent);
}

// =========================
// Concrete tools
// =========================

// Spell checker
pub struct SpellCheck;

impl Tool for SpellCheck {
    fn apply(&self, context: &mut EditorContent) {
        context.content.push_str("\n[Spell check OK]");
    }
}

// Git integration
pub struct Git;

impl Tool for Git {
    fn apply(&self, context: &mut EditorContent) {
        context.content.push_str("\n[Git status clean]");
    }
}

// =========================
// Usage
// =========================

fn main() {
    let editor = Editor;
    let spellcheck = SpellCheck;
    let git = Git;

    let mut context = EditorContent {
        content: String::from("Hello world"),
    };

    editor.run(&spellcheck, &git, &mut context);

    println!("--- FINAL CONTENT ---");
    println!("{}", context.content);
}
