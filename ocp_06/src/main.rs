// cargo run -p ex_06_ocp
// ! DOES NOT COMPILE

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

// Editor using static dispatch
pub struct Editor<T: Tool> {
    tools: Vec<T>,
}

impl<T: Tool> Editor<T> {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn register_tool(&mut self, tool: T) {
        self.tools.push(tool);
    }

    pub fn run(&mut self, context: &mut EditorContext) {
        for tool in &mut self.tools {
            println!("Running tool: {}", tool.name());
            tool.apply(context); // Direct call, no vtable
        }
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
    let mut editor = Editor::new();

    // Tools must be of the same concrete type T
    editor.register_tool(SpellCheck);
    editor.register_tool(Git);

    let mut context = EditorContext {
        content: String::from("HELLO WORLD"),
    };

    editor.run(&mut context);

    println!("--- FINAL CONTENT ---");
    println!("{}", context.content);
}
