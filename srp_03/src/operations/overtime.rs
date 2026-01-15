// src/operations/overtime.rs
// Operations' responsibility

use crate::domain::employee::Employee;

pub struct OvertimeTracker;

impl OvertimeTracker {
    pub fn calculate_overtime_hours(employee: &Employee) -> f64 {
        (employee.hours_worked - 40.0).max(0.0)
    }
}
