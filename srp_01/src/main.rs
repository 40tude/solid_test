// cargo run -p ex_01_srp

use std::fmt;

/// Dummy database error type
#[derive(Debug)]
pub struct DbError;

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Database error")
    }
}

/// Dummy database abstraction
pub struct Database;

impl Database {
    /// Execute a fake SQL query
    pub fn execute(&mut self, query: &str, params: &[&dyn fmt::Debug]) -> Result<(), DbError> {
        println!("Executing SQL: {}", query);
        println!("With params: {:?}", params);
        Ok(())
    }
}

pub struct Employee {
    pub id: u32,
    pub name: String,
    pub hours_worked: f64,
    pub rate: f64,
}

impl Employee {
    /// Calculate pay (used by Accounting)
    pub fn calculate_pay(&self) -> f64 {
        let regular_hours = self.hours_worked.min(40.0);
        let overtime_hours = (self.hours_worked - 40.0).max(0.0);
        regular_hours * self.rate + overtime_hours * self.rate * 1.5
    }

    /// Calculate overtime hours for operations report (used by Operations)
    pub fn calculate_overtime_hours(&self) -> f64 {
        (self.hours_worked - 40.0).max(0.0)
    }

    /// Save to database (used by DBAs / Infrastructure)
    pub fn save(&self, db: &mut Database) -> Result<(), DbError> {
        db.execute(
            "INSERT INTO employees VALUES (?, ?, ?, ?)",
            &[&self.id, &self.name, &self.hours_worked, &self.rate],
        )
    }

    /// Generate report (used by HR)
    pub fn generate_report(&self) -> String {
        format!(
            "Employee Report\n\
             Name: {}\n\
             Hours: {}\n\
             Pay: ${:.2}",
            self.name,
            self.hours_worked,
            self.calculate_pay()
        )
    }
}

fn main() {
    let employee = Employee {
        id: 1,
        name: "Alice".to_string(),
        hours_worked: 45.0,
        rate: 20.0,
    };

    // Accounting client
    let pay = employee.calculate_pay();
    println!("Accounting: pay = ${:.2}", pay);

    // Operations client
    let overtime = employee.calculate_overtime_hours();
    println!("Operations: overtime hours = {}", overtime);

    // Infrastructure / DBA client
    let mut db = Database;
    employee.save(&mut db).unwrap();

    // HR client
    let report = employee.generate_report();
    println!("\nHR Report:\n{}", report);
}
