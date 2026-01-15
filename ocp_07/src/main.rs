// cargo run -p ex_07_ocp

// =========================
// Static dispatch example
// =========================

// Shared editor state
pub struct EditorContext {
    pub content: String,
}

// Compile-time behavior abstraction
pub trait Tool {
    fn name(&self) -> &str;
    fn apply(&mut self, context: &mut EditorContext);
}

// Apply a tool or a chain of tools
pub trait ToolChain {
    fn apply(&mut self, context: &mut EditorContext);
}

// Implementation for a unique tool
// A single Tool is also a valid ToolChain
// If we don't have this implementation there is no way to implement the recusrsive
impl<T: Tool> ToolChain for T {
    fn apply(&mut self, context: &mut EditorContext) {
        // self.apply(context); // CANNOT work: .apply() calls .apply()
        Tool::apply(self, context);
    }
}

// Recusrsive implementation for a tuple
// A tuple (Head, Tail) is a ToolChain if:
//      Head is a Tool
//      Tail is already a ToolChain
impl<Head, Tail> ToolChain for (Head, Tail)
where
    Head: Tool,
    Tail: ToolChain,
{
    fn apply(&mut self, context: &mut EditorContext) {
        self.0.apply(context);
        self.1.apply(context);
    }
}

// Editor using static dispatch
pub struct Editor<T> {
    tools: T,
}

impl<T: ToolChain> Editor<T> {
    pub fn new(tools: T) -> Self {
        Self { tools }
    }

    pub fn run(&mut self, context: &mut EditorContext) {
        self.tools.apply(context);
    }
}

// Spell check tool
pub struct SpellCheck;

impl Tool for SpellCheck {
    fn name(&self) -> &str {
        "Spell Checker"
    }

    fn apply(&mut self, context: &mut EditorContext) {
        // Normalize text to simulate spell checking
        context.content = context.content.to_lowercase();
        context.content.push_str("\n[Spell check OK]");
    }
}

// Git integration tool
pub struct Git;

impl Tool for Git {
    fn name(&self) -> &str {
        "Git Integration"
    }

    fn apply(&mut self, context: &mut EditorContext) {
        context.content.push_str("\n[Git status clean]");
    }
}

fn main() {
    let mut editor = Editor::new((SpellCheck, Git));

    // At this point the chain is complete:
    // Git implements Tool
    // therefore Git implements ToolChain
    // therefore (SpellCheck, Git) implements ToolChain
    // therefore Editor<(SpellCheck, Git)> is valid

    let mut context = EditorContext {
        content: String::from("HELLO WORLD"),
    };

    editor.run(&mut context);

    println!("--- FINAL CONTENT ---");
    println!("{}", context.content);
}
