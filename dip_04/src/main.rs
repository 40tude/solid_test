// cargo run -p ex_04_dip

// =========================
// Hexagonal Architecture - aka Ports & Adapters
// =========================

// DOMAIN Layer (Core Business Logic)
mod domain {
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
}

// PORTS (Abstractions defined by domain)
mod ports {
    use crate::domain::*;

    // Output port: domain needs to persist orders
    pub trait OrderRepository {
        fn save(&mut self, order: &Order) -> Result<(), OrderError>;
        fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError>;
    }

    // Output port: domain needs to process payments
    pub trait PaymentGateway {
        fn charge(&self, amount: Money) -> Result<(), OrderError>;
    }

    // Output port: domain needs to send notifications
    pub trait NotificationService {
        fn send_confirmation(&self, order: &Order) -> Result<(), OrderError>;
    }
}

// APPLICATION service (Orchestrates domain + ports)
mod application {
    use crate::domain::*;
    use crate::ports::*;

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
            // Pure business logic - no infrastructure concerns!
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
}

// ADAPTERS - Implementation #1 (In-Memory)
mod in_memory_adapters {
    use crate::domain::*;
    use crate::ports::*;
    use std::collections::HashMap;

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
}

// ADAPTERS - Implementation #2 (Simulated External Services)
mod external_adapters {
    use crate::domain::*;
    use crate::ports::*;
    use std::collections::HashMap;

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
            println!("[SendGrid API] POST /mail/send to=customer@example.com subject='Order #{:?} Confirmed'",
                order.id);
            Ok(())
        }
    }
}

// Demonstrating Swappable Adapters
fn main() {
    use application::OrderService;
    use domain::{LineItem, Money, OrderId};
    use external_adapters::*;
    use in_memory_adapters::*;

    println!("=== Hexagonal Architecture Demo ===\n");

    // Create test items
    let items = vec![
        LineItem {
            name: "Rust Programming Book".to_string(),
            price: Money(4999), // $49.99
        },
        LineItem {
            name: "Mechanical Keyboard".to_string(),
            price: Money(12999), // $129.99
        },
    ];

    println!("--- Configuration #1: In-Memory Adapters (Testing) ---\n");
    {
        let repo = InMemoryOrderRepository::new();
        let payment = MockPaymentGateway;
        let notifications = ConsoleNotificationService;

        let mut service = OrderService::new(repo, payment, notifications);

        match service.place_order(items.clone()) {
            Ok(order) => println!("Order placed successfully: {:?}\n", order.id),
            Err(e) => println!("Error: {}\n", e),
        }
    }

    println!("--- Configuration #2: External Services (Production) ---\n");
    {
        let repo = PostgresOrderRepository::new();
        let payment = StripePaymentGateway;
        let notifications = SendGridNotificationService;

        let mut service = OrderService::new(repo, payment, notifications);

        match service.place_order(items.clone()) {
            Ok(order) => {
                println!("Order placed successfully: {:?}", order.id);

                // Demonstrate retrieval
                println!();
                if let Ok(Some(retrieved)) = service.get_order(order.id) {
                    println!(
                        "Retrieved order: {} items, total ${}.{:02}",
                        retrieved.items.len(),
                        retrieved.total.0 / 100,
                        retrieved.total.0 % 100
                    );
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
