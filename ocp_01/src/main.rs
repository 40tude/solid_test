// cargo run -p ex_01_ocp

// =========================
// Naïve Solution - Reporter
// =========================

// This enum defines all supported report formats
// Adding a new format requires modifying this enum
pub enum ReportFormat {
    Text,
    Html,
    Pdf,
}

// This struct represents the report data
pub struct Report {
    title: String,
    data: Vec<String>,
}

impl Report {
    // This method violates the Open-Closed Principle
    // Each new format requires modifying this method
    pub fn generate(&self, format: ReportFormat) -> String {
        match format {
            ReportFormat::Text => {
                let mut output = format!("=== {} ===", self.title);
                for item in &self.data {
                    output.push_str(&format!("\n- {}", item));
                }
                output
            }
            ReportFormat::Html => {
                let mut output = format!("<h1>{}</h1>\n<ul>\n", self.title);
                for item in &self.data {
                    output.push_str(&format!("  <li>{}</li>\n", item));
                }
                output.push_str("</ul>");
                output
            }
            ReportFormat::Pdf => {
                // Fake PDF generation
                format!("PDF: {} [binary data]", self.title)
            }
        }
    }
}

fn main() {
    // Sample report data
    let report = Report {
        title: String::from("Monthly Sales"),
        data: vec![
            String::from("Product A: 120 units"),
            String::from("Product B: 98 units"),
            String::from("Product C: 143 units"),
        ],
    };

    // Generate reports in different formats
    let text_report = report.generate(ReportFormat::Text);
    let html_report = report.generate(ReportFormat::Html);
    let pdf_report = report.generate(ReportFormat::Pdf);

    println!("\n--- TEXT REPORT ---\n{}", text_report);
    println!("\n--- HTML REPORT ---\n{}", html_report);
    println!("\n--- PDF REPORT ---\n{}", pdf_report);
}
