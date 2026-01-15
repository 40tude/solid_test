// src/main.rs
// cargo run -p ex_03_srp

mod accounting;
mod domain;
mod hr;
mod infrastructure;
mod operations;

fn main() {
    use crate::accounting::payroll::PayrollCalculator;
    use crate::domain::employee::Employee;
    use crate::hr::reporting::EmployeeReporter;
    use crate::infrastructure::db::Database;
    use crate::infrastructure::repository::EmployeeRepository;
    use crate::operations::overtime::OvertimeTracker;

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
