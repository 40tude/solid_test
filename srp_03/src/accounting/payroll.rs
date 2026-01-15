// src/accounting/payroll.rs
// Accounting's responsibility

use crate::domain::employee::Employee;

pub struct PayrollCalculator;

impl PayrollCalculator {
    pub fn calculate_pay(employee: &Employee) -> f64 {
        let regular_hours = employee.hours_worked.min(40.0);
        let overtime_hours = (employee.hours_worked - 40.0).max(0.0);
        regular_hours * employee.rate + overtime_hours * employee.rate * 1.5
    }
}
