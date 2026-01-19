//! Ports - Abstractions defined by domain
//!
//! These traits define the interfaces that the domain needs.
//! Implementations (adapters) are provided externally.

use crate::domain::{Money, Order, OrderError, OrderId};

/// Output port: domain needs to persist orders
pub trait OrderRepository {
    fn save(&mut self, order: &Order) -> Result<(), OrderError>;
    fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError>;
}

/// Output port: domain needs to process payments
pub trait PaymentGateway {
    fn charge(&self, amount: Money) -> Result<(), OrderError>;
}

/// Output port: domain needs to send notifications
pub trait NotificationService {
    fn send_confirmation(&self, order: &Order) -> Result<(), OrderError>;
}
