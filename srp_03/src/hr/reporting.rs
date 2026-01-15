// src/hr/reporting.rs
// HR's responsibility

use crate::accounting::payroll::PayrollCalculator;
use crate::domain::employee::Employee;

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
