// cargo run -p ex_02_srp

use std::fmt;

// Domain
// This module should be in: src/domain/employee.rs
// Core data - this is just data, no behavior
mod domain {
    pub struct Employee {
        pub id: u32,
        pub name: String,
        pub hours_worked: f64,
        pub rate: f64,
    }
}

// Infrastructure/DBA's responsibility
mod infrastructure {
    // This module should be in: src/infrastructure/db.rs
    pub mod db {
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
            pub fn execute(&self, query: &str, params: &[&dyn fmt::Debug]) -> Result<(), DbError> {
                println!("Executing SQL: {}", query);
                println!("With params: {:?}", params);
                Ok(())
            }
        }
    }

    // This module should be in: src/infrastructure/repository.rs
    pub mod repository {
        use super::db::{Database, DbError};
        use crate::domain::Employee;

        /// Infrastructure / DBA's responsibility
        pub struct EmployeeRepository {
            pub db: Database,
        }

        impl EmployeeRepository {
            pub fn save(&self, employee: &Employee) -> Result<(), DbError> {
                self.db.execute(
                    "INSERT INTO employees VALUES (?, ?, ?, ?)",
                    &[
                        &employee.id,
                        &employee.name,
                        &employee.hours_worked,
                        &employee.rate,
                    ],
                )
            }

            pub fn find_by_id(&self, _id: u32) -> Result<Employee, DbError> {
                // Fake implementation for demo purposes
                Ok(Employee {
                    id: 1,
                    name: "Alice".to_string(),
                    hours_worked: 40.0,
                    rate: 20.0,
                })
            }
        }
    }
}

// This module should be in: src/accounting/payroll.rs
// Accounting's responsibility
mod accounting {
    use crate::domain::Employee;

    pub struct PayrollCalculator;

    impl PayrollCalculator {
        pub fn calculate_pay(employee: &Employee) -> f64 {
            let regular_hours = employee.hours_worked.min(40.0);
            let overtime_hours = (employee.hours_worked - 40.0).max(0.0);
            regular_hours * employee.rate + overtime_hours * employee.rate * 1.5
        }
    }
}

// This module should be in: src/operations/overtime.rs
// Operations' responsibility
mod operations {
    use crate::domain::Employee;

    pub struct OvertimeTracker;

    impl OvertimeTracker {
        pub fn calculate_overtime_hours(employee: &Employee) -> f64 {
            (employee.hours_worked - 40.0).max(0.0)
        }
    }
}

// This module should be in: src/hr/reporting.rs
// HR's responsibility
mod hr {
    use crate::accounting::PayrollCalculator;
    use crate::domain::Employee;

    /// HR's responsibility
    pub struct EmployeeReporter;

    impl EmployeeReporter {
        pub fn generate_text_report(employee: &Employee) -> String {
            format!(
                "Employee Report\n\
                 Name: {}\n\
                 Hours: {}\n\
                 Pay: ${:.2}",
                employee.name,
                employee.hours_worked,
                PayrollCalculator::calculate_pay(employee)
            )
        }

        pub fn generate_json_report(employee: &Employee) -> String {
            format!(
                r#"{{"name": "{}", "hours": {}, "pay": {:.2}}}"#,
                employee.name,
                employee.hours_worked,
                PayrollCalculator::calculate_pay(employee)
            )
        }
    }
}

// This should be in: src/main.rs
fn main() {
    use accounting::PayrollCalculator;
    use domain::Employee;
    use hr::EmployeeReporter;
    use infrastructure::db::Database;
    use infrastructure::repository::EmployeeRepository;
    use operations::OvertimeTracker;

    let employee = Employee {
        id: 1,
        name: "Alice".to_string(),
        hours_worked: 45.0,
        rate: 20.0,
    };

    // Accounting client
    let pay = PayrollCalculator::calculate_pay(&employee);
    println!("Accounting: pay = ${:.2}", pay);

    // Operations client
    let overtime = OvertimeTracker::calculate_overtime_hours(&employee);
    println!("Operations: overtime hours = {}", overtime);

    // Infrastructure client
    let repo = EmployeeRepository { db: Database };
    repo.save(&employee).unwrap();

    // HR client
    println!(
        "\nHR Text Report:\n{}",
        EmployeeReporter::generate_text_report(&employee)
    );
    println!(
        "\nHR JSON Report:\n{}",
        EmployeeReporter::generate_json_report(&employee)
    );
}
