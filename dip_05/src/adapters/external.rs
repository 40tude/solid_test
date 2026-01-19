//! External Adapters - Simulated production services

use std::collections::HashMap;

use crate::domain::{Money, Order, OrderError, OrderId};
use crate::ports::{NotificationService, OrderRepository, PaymentGateway};

pub struct PostgresOrderRepository {
    simulated_db: HashMap<OrderId, Order>,
}

impl PostgresOrderRepository {
    pub fn new() -> Self {
        Self {
            simulated_db: HashMap::new(),
        }
    }
}

impl Default for PostgresOrderRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderRepository for PostgresOrderRepository {
    fn save(&mut self, order: &Order) -> Result<(), OrderError> {
        println!("[Postgres] INSERT INTO orders VALUES ({:?}, ...)", order.id);
        self.simulated_db.insert(order.id, order.clone());
        Ok(())
    }

    fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
        println!("[Postgres] SELECT * FROM orders WHERE id = {:?}", id);
        Ok(self.simulated_db.get(&id).cloned())
    }
}

pub struct StripePaymentGateway;

impl PaymentGateway for StripePaymentGateway {
    fn charge(&self, amount: Money) -> Result<(), OrderError> {
        println!(
            "[Stripe API] POST /charges amount=${}.{:02}",
            amount.0 / 100,
            amount.0 % 100
        );
        Ok(())
    }
}

pub struct SendGridNotificationService;

impl NotificationService for SendGridNotificationService {
    fn send_confirmation(&self, order: &Order) -> Result<(), OrderError> {
        println!(
            "[SendGrid API] POST /mail/send to=customer@example.com subject='Order #{:?} Confirmed'",
            order.id
        );
        Ok(())
    }
}
