//! In-Memory Adapters - For testing and development

use std::collections::HashMap;

use crate::domain::{Money, Order, OrderError, OrderId};
use crate::ports::{NotificationService, OrderRepository, PaymentGateway};

pub struct InMemoryOrderRepository {
    orders: HashMap<OrderId, Order>,
}

impl InMemoryOrderRepository {
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
        }
    }
}

impl Default for InMemoryOrderRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderRepository for InMemoryOrderRepository {
    fn save(&mut self, order: &Order) -> Result<(), OrderError> {
        println!("[InMemory] Saving order #{:?}", order.id);
        self.orders.insert(order.id, order.clone());
        Ok(())
    }

    fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
        println!("[InMemory] Finding order #{:?}", id);
        Ok(self.orders.get(&id).cloned())
    }
}

pub struct MockPaymentGateway;

impl PaymentGateway for MockPaymentGateway {
    fn charge(&self, amount: Money) -> Result<(), OrderError> {
        println!("[Mock] Charging ${}.{:02}", amount.0 / 100, amount.0 % 100);
        Ok(())
    }
}

pub struct ConsoleNotificationService;

impl NotificationService for ConsoleNotificationService {
    fn send_confirmation(&self, order: &Order) -> Result<(), OrderError> {
        println!(
            "[Console] Order #{:?} confirmed - Total: ${}.{:02}",
            order.id,
            order.total.0 / 100,
            order.total.0 % 100
        );
        Ok(())
    }
}
