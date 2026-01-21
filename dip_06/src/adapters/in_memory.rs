// =============================================================================
// IN-MEMORY ADAPTERS - Your Testing Best Friends
// =============================================================================
//
// These adapters are designed for testing and local development.
// No database connections, no network calls, no external dependencies.
// Just plain Rust data structures.
//
// "But wait," you might say, "why do I need fake implementations?"
//
// Great question! Here's why:
//
// 1. UNIT TESTS: You want to test OrderService logic without spinning up
//    a PostgreSQL database. These adapters let you do that.
//
// 2. LOCAL DEVELOPMENT: Start coding without setting up infrastructure.
//    Just run `cargo run` and everything works.
//
// 3. CI/CD PIPELINES: No need to configure databases in your build server.
//    Tests run fast and reliably.
//
// 4. DEMOS AND PROTOTYPES: Show stakeholders a working app without
//    worrying about production infrastructure.
//
// Remember dip_02? The Email struct was our first adapter.
// These are the same concept, just for different ports.

// =============================================================================
// Imports - The Dependency Direction
// =============================================================================
// Look carefully at what we import:

use std::collections::HashMap;

use crate::domain::{Money, Order, OrderError, OrderId};
use crate::ports::{OrderRepository, PaymentGateway, Sender};

// We import:
// - Standard library (HashMap for storage)
// - Domain types (Order, Money, etc.)
// - Port traits (the interfaces we're implementing)
//
// We DON'T import:
// - Anything from application/
// - Anything from external.rs
// - Any external crates (no sqlx, no reqwest)
//
// This adapter is PURE RUST. It could be compiled and tested
// without any external dependencies at all.

// =============================================================================
// InMemoryOrderRepository - A HashMap Pretending to Be a Database
// =============================================================================
//
// This is the simplest possible implementation of OrderRepository.
// A HashMap is our "database". When the process exits, data is gone.
// And that's perfectly fine for testing!

pub struct InMemoryOrderRepository {
    // The "database" - just a HashMap in memory.
    // HashMap<OrderId, Order> means: "map order IDs to orders"
    orders: HashMap<OrderId, Order>,
}

impl InMemoryOrderRepository {
    /// Creates a new empty repository.
    ///
    /// In production code (PostgresOrderRepository), this might take
    /// connection strings, pool configurations, etc.
    /// Here? Nothing. Just an empty HashMap.
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
        }
    }
}

// Rust convention: if you have new(), also implement Default.
// This allows users to write: `let repo = InMemoryOrderRepository::default();`
impl Default for InMemoryOrderRepository {
    fn default() -> Self {
        Self::new()
    }
}

// Here's where we implement the port trait!
// This is the contract fulfillment: "I promise to provide save() and find()."
impl OrderRepository for InMemoryOrderRepository {
    /// Saves an order to our HashMap "database".
    ///
    /// In a real database, this would be: INSERT INTO orders (...)
    /// Here, it's just: HashMap.insert()
    ///
    /// Note: we clone the order because HashMap takes ownership,
    /// but we only have a reference. In a real DB, you'd serialize it.
    fn save(&mut self, order: &Order) -> Result<(), OrderError> {
        println!("  [InMemory] Saving order #{:?}", order.id);
        self.orders.insert(order.id, order.clone());
        Ok(())
    }

    /// Finds an order by ID.
    ///
    /// In a real database: SELECT * FROM orders WHERE id = ?
    /// Here: HashMap.get()
    ///
    /// Returns Option<Order>: Some if found, None if not.
    /// We clone because we can't move data out of the HashMap.
    fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
        println!("  [InMemory] Finding order #{:?}", id);
        Ok(self.orders.get(&id).cloned())
    }
}

// =============================================================================
// MockPaymentGateway - Always Says "Payment Successful!"
// =============================================================================
//
// This is a "happy path" mock. It always succeeds.
// Perfect for testing the normal flow of your application.
//
// In a more sophisticated test setup, you might have:
// - FailingPaymentGateway (always fails)
// - FlakeyPaymentGateway (fails randomly)
// - SlowPaymentGateway (adds delays)
//
// Each helps test different scenarios without touching real payment APIs.

pub struct MockPaymentGateway;

impl PaymentGateway for MockPaymentGateway {
    /// "Charges" the amount by... printing a message.
    ///
    /// No real money moves. No API calls. Just a log line.
    /// But from OrderService's perspective, the contract is fulfilled:
    /// "I called charge(), it returned Ok(). Payment done!"
    fn charge(&self, amount: Money) -> Result<(), OrderError> {
        println!(
            "  [Mock] Charging ${}.{:02}",
            amount.0 / 100,
            amount.0 % 100
        );
        Ok(())
    }
}

// =============================================================================
// ConsoleSender - Notifications Go to stdout
// =============================================================================
//
// Remember the Email struct from dip_02? This is its spiritual successor.
// Instead of pretending to send emails, we just print to the console.
//
// The name follows the pattern: Console + Sender
// - Console = where it sends (stdout)
// - Sender = the trait it implements
//
// You could also have: FileSender, LogSender, NullSender (discards everything)

pub struct ConsoleSender;

impl Sender for ConsoleSender {
    /// "Sends" a notification by printing to the console.
    ///
    /// In production, this might:
    /// - Call SendGrid API
    /// - Queue a message in RabbitMQ
    /// - Send an SMS via Twilio
    ///
    /// Here, it just prints. And that's enough for testing!
    fn send(&self, order: &Order) -> Result<(), OrderError> {
        println!(
            "  [Console] Order #{:?} confirmed! Total: ${}.{:02}",
            order.id,
            order.total.0 / 100,
            order.total.0 % 100
        );
        Ok(())
    }
}

// =============================================================================
// Testing with These Adapters
// =============================================================================
//
// Here's how you'd write a unit test using these adapters:
//
//     #[test]
//     fn test_place_order() {
//         let repo = InMemoryOrderRepository::new();
//         let payment = MockPaymentGateway;
//         let sender = ConsoleSender;
//
//         let mut service = OrderService::new(repo, payment, sender);
//
//         let items = vec![LineItem { name: "Test".into(), price: Money(100) }];
//         let result = service.place_order(items);
//
//         assert!(result.is_ok());
//     }
//
// No database setup. No mock servers. No network. Just fast, reliable tests.
//
// For more advanced testing, you could create adapters that record calls:
//
//     struct SpyPaymentGateway {
//         charges: RefCell<Vec<Money>>,
//     }
//
// Then assert that charge() was called with the right amount.
// The possibilities are endless when your architecture supports it!

// =============================================================================
// Key Takeaway
// =============================================================================
//
// These adapters are intentionally SIMPLE. They're not production code.
// They exist to:
//
// 1. Prove that the ports work (you can implement them)
// 2. Enable testing without infrastructure
// 3. Allow rapid development and prototyping
//
// The simplicity is the point. When you see how easy it is to create
// a fake implementation, you realize the power of DIP:
//
// Your business logic (domain + application) doesn't care about
// implementation details. It works the same with a HashMap or PostgreSQL.
//
// Next: check out external.rs for "production-like" implementations!
