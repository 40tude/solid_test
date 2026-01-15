// src/domain/employee.rs
// Core data - this is just data, no behavior
pub struct Employee {
    pub id: u32,
    pub name: String,
    pub hours_worked: f64,
    pub rate: f64,
}
