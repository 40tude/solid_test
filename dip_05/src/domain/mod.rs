//! Domain Layer - Core Business Logic
//!
//! Contains pure business entities and rules with no external dependencies.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u32);

#[derive(Debug, Clone, Copy)]
pub struct Money(pub u32); // cents

#[derive(Debug, Clone)]
pub struct LineItem {
    pub name: String,
    pub price: Money,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub items: Vec<LineItem>,
    pub total: Money,
}

#[derive(Debug)]
pub enum OrderError {
    InvalidOrder,
    PaymentFailed,
    StorageFailed,
    NotificationFailed,
}

impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Order {
    pub fn new(id: OrderId, items: Vec<LineItem>) -> Result<Self, OrderError> {
        if items.is_empty() {
            return Err(OrderError::InvalidOrder);
        }

        let total = Money(items.iter().map(|item| item.price.0).sum());

        Ok(Order { id, items, total })
    }
}
