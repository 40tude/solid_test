// cargo run -p ex_03_ocp

// =========================
// Abstractions
// =========================

// The report doesn't know about specific formats
pub struct Report {
    title: String,
    data: Vec<String>,
}

// However, the report has a generate method which calls the .format() method of the formatter
// The call will be resolve at compile time
impl Report {
    // Generic version using static dispatch
    pub fn generate<F: ReportFormatter>(&self, formatter: &F) -> String {
        formatter.format(&self.title, &self.data)
    }
}

// If a type wants to have the ReportFormatter trait it must implement the format method
pub trait ReportFormatter {
    fn format(&self, title: &str, data: &[String]) -> String;
}

// =========================
// Concrete formatters
// =========================

// Plain text output
pub struct TextFormatter;

impl ReportFormatter for TextFormatter {
    fn format(&self, title: &str, data: &[String]) -> String {
        let mut output = format!("=== {} ===\n", title);
        for item in data {
            output.push_str(&format!("- {}\n", item));
        }
        output
    }
}

// HTML output
pub struct HtmlFormatter;

impl ReportFormatter for HtmlFormatter {
    fn format(&self, title: &str, data: &[String]) -> String {
        let mut output = format!("<h1>{}</h1>\n<ul>\n", title);
        for item in data {
            output.push_str(&format!("  <li>{}</li>\n", item));
        }
        output.push_str("</ul>");
        output
    }
}

// Fake PDF output
pub struct PdfFormatter;

impl ReportFormatter for PdfFormatter {
    fn format(&self, title: &str, _data: &[String]) -> String {
        format!("PDF: {} [binary data]", title)
    }
}

// XML output
pub struct XmlFormatter;

impl ReportFormatter for XmlFormatter {
    fn format(&self, title: &str, data: &[String]) -> String {
        let mut output = String::from("<report>\n");
        output.push_str(&format!("  <title>{}</title>\n", title));
        output.push_str("  <items>\n");

        for item in data {
            output.push_str(&format!("    <item>{}</item>\n", item));
        }

        output.push_str("  </items>\n</report>");
        output
    }
}

// =========================
// Usage
// =========================

fn main() {
    let report = Report {
        title: "Monthly Sales".to_string(),
        data: vec![
            "Product A: 120 units".to_string(),
            "Product B: 98 units".to_string(),
            "Product C: 143 units".to_string(),
        ],
    };

    println!("--- TEXT ---\n{}", report.generate(&TextFormatter));
    println!("--- HTML ---\n{}", report.generate(&HtmlFormatter));
    println!("--- PDF ---\n{}", report.generate(&PdfFormatter));
    println!("--- XML ---\n{}", report.generate(&XmlFormatter));
}
