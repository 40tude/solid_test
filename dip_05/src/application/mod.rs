//! Application Layer - Use Cases / Service Orchestration
//!
//! Orchestrates domain logic and ports. Contains no business rules,
//! only coordinates the flow between domain and external services.

use crate::domain::{LineItem, Order, OrderError, OrderId};
use crate::ports::{NotificationService, OrderRepository, PaymentGateway};

pub struct OrderService<R, P, N>
where
    R: OrderRepository,
    P: PaymentGateway,
    N: NotificationService,
{
    repository: R,
    payment: P,
    notifications: N,
    next_id: u32,
}

impl<R, P, N> OrderService<R, P, N>
where
    R: OrderRepository,
    P: PaymentGateway,
    N: NotificationService,
{
    pub fn new(repository: R, payment: P, notifications: N) -> Self {
        Self {
            repository,
            payment,
            notifications,
            next_id: 1,
        }
    }

    pub fn place_order(&mut self, items: Vec<LineItem>) -> Result<Order, OrderError> {
        let order_id = OrderId(self.next_id);
        self.next_id += 1;

        let order = Order::new(order_id, items)?;

        // Use abstractions, not concrete implementations
        self.payment.charge(order.total)?;
        self.repository.save(&order)?;
        self.notifications.send_confirmation(&order)?;

        Ok(order)
    }

    pub fn get_order(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
        self.repository.find(id)
    }
}
