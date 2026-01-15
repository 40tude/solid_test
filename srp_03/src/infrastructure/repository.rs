// src/infrastructure/repository.rs
// Infrastructure / DBA's responsibility

use super::db::{Database, DbError};
use crate::domain::employee::Employee;

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
