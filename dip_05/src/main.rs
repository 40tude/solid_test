// cargo run -p ex_05_dip

// Welcome back! In ex_02_dip, we saw how to invert dependencies using a trait.
// We had one trait (Sender) and one adapter (Email). Simple and clean.
// But real applications are more complex. They need databases, payment systems,
// notification services... How do we scale DIP to handle all of that?
//
// The answer: Hexagonal Architecture, also known as "Ports & Adapters".
// Don't let the fancy name scare you. It's just DIP applied systematically.
// Let's break it down together.

// =============================================================================
// DOMAIN Layer - The Heart of our Application
// =============================================================================
// Remember in ex_02_dip, domain contained both our business entity (OrderService)
// AND our abstraction (the Sender trait)?
//
// In Hexagonal, we split things more carefully. The domain module becomes
// purely about business concepts: What is an Order? What is Money?
// No traits here: just the core vocabulary of our business.
mod domain {
    use std::fmt;

    // These are "Value Objects": they represent business concepts.
    // OrderId isn't just a u32, it's a meaningful business identifier.
    // This makes our code speak the language of the business.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct OrderId(pub u32);

    #[derive(Debug, Clone, Copy)]
    pub struct Money(pub u32); // cents

    #[derive(Debug, Clone)]
    pub struct LineItem {
        pub name: String,
        pub price: Money,
    }

    // Our Order entity: this is pure business logic.
    // Notice: no database stuff, no HTTP, no external dependencies.
    // Just: "What IS an order?"
    #[derive(Debug, Clone)]
    pub struct Order {
        pub id: OrderId,
        pub items: Vec<LineItem>,
        pub total: Money,
    }

    // Business errors: things that can go wrong in our domain.
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

    // Business rule: an order must have at least one item.
    // This validation lives in the domain: it's a business rule,
    // not a database constraint or an API validation.
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

// =============================================================================
// PORTS - The Boundaries of Our Domain
// =============================================================================
// Here's where it gets interesting. Remember the Sender trait from ex_02_dip?
// It was our way of saying: "I need to send messages, but I don't care how."
//
// In Hexagonal, we call these abstractions "Ports". They're the plugs
// where external systems connect to our application.
//
// Why a separate module? Because ports are contracts. They belong to the domain
// conceptually (the domain DEFINES what it needs), but separating them makes
// the architecture crystal clear: domain = business concepts, ports = boundaries.
mod ports {
    use crate::domain::*;

    // Output port: "I need to store orders somewhere"
    // Could be PostgreSQL, MongoDB, a file, Redis... domain doesn't care.
    pub trait OrderRepository {
        fn save(&mut self, order: &Order) -> Result<(), OrderError>;
        fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError>;
    }

    // Output port: "I need to charge customers"
    // Could be Stripe, PayPal, a mock for testing... domain doesn't care.
    pub trait PaymentGateway {
        fn charge(&self, amount: Money) -> Result<(), OrderError>;
    }

    // Output port: "I need to notify customers"
    // Hey, look! It's our old friend Sender from ex_02_dip.
    // Same concept, just living in a dedicated "ports" module now.
    // Could be Email, SMS, push notifications, carrier pigeon...
    pub trait Sender {
        fn send(&self, order: &Order) -> Result<(), OrderError>;
    }
}

// =============================================================================
// APPLICATION Layer - The Orchestrator
// =============================================================================
// In ex_02_dip, OrderService was in the domain module.
// Here, we move it to a separate "application" layer. Why?
//
// Because OrderService doesn't define business rules: it ORCHESTRATES them.
// It's the conductor: "First charge the payment, then save the order,
// then send a notification." That's coordination, not business logic.
//
// This separation helps when our app grows: domain stays focused on
// "what things ARE", application handles "what happens when".
mod application {
    use crate::domain::*;
    use crate::ports::*;

    // Look familiar? It's our OrderService, but now with THREE dependencies!
    // In ex_02_dip, we had: OrderService<S: Sender>
    // ! Now we have: OrderService<R: OrderRepository, P: PaymentGateway, N: Sender>
    //
    // Same principle, just more ports. Each one is a plug where we can
    // connect different adapters.
    pub struct OrderService<'a, R, P, N>
    where
        R: OrderRepository,
        P: PaymentGateway,
        N: Sender,
    {
        repository: &'a mut R,
        payment: &'a P,
        sender: &'a N,
        next_id: u32,
    }

    impl<'a, R, P, N> OrderService<'a, R, P, N>
    where
        R: OrderRepository,
        P: PaymentGateway,
        N: Sender,
    {
        // Dependency Injection: same as ex_02_dip, just with more dependencies.
        // The caller decides which implementations to use.
        pub fn new(repository: &'a mut R, payment: &'a P, sender: &'a N) -> Self {
            Self {
                repository,
                payment,
                sender,
                next_id: 1,
            }
        }

        // This is the "use case": what happens when a customer places an order.
        // Notice: we only talk to abstractions (traits), never to concrete types.
        // We don't know if we're using Postgres or an in-memory HashMap.
        // We don't know if we're charging via Stripe or a mock.
        // And that's exactly the point!
        pub fn place_order(&mut self, items: Vec<LineItem>) -> Result<Order, OrderError> {
            let order_id = OrderId(self.next_id);
            self.next_id += 1;

            // Pure business logic: create the order
            let order = Order::new(order_id, items)?;

            // Orchestration: coordinate the external systems
            // Each of these calls goes through a port (trait)
            self.payment.charge(order.total)?;
            self.repository.save(&order)?;
            self.sender.send(&order)?;

            Ok(order)
        }

        pub fn get_order(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
            self.repository.find(id)
        }
    }
}

// =============================================================================
// ADAPTERS - The Implementations
// =============================================================================
// Now for the fun part: Adapters!
// These are the concrete implementations that plug into our ports.
//
// In ex_02_dip, Email was our adapter: it implemented the Sender trait.
// Here, we have multiple adapters for each port.
//
// Key insight: adapters depend on ports, not the other way around.
// The dependency arrow points INWARD, toward the domain. That's DIP in action!

// --- Adapter Set #1: In-Memory (for testing and development) ---
mod in_memory_adapters {
    use crate::domain::*;
    use crate::ports::*;
    use std::collections::HashMap;

    // A simple HashMap-based repository.
    // Perfect for unit tests: no database needed!
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

    // It implements the OrderRepository port.
    // The application layer doesn't know (or care) that this is a HashMap.
    impl OrderRepository for InMemoryOrderRepository {
        fn save(&mut self, order: &Order) -> Result<(), OrderError> {
            println!("  [InMemory] Saving order #{:?}", order.id);
            self.orders.insert(order.id, order.clone());
            Ok(())
        }

        fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
            println!("  [InMemory] Finding order #{:?}", id);
            Ok(self.orders.get(&id).cloned())
        }
    }

    // A mock payment gateway: always succeeds.
    // Great for testing the happy path!
    pub struct MockPaymentGateway;

    impl PaymentGateway for MockPaymentGateway {
        fn charge(&self, amount: Money) -> Result<(), OrderError> {
            println!(
                "  [Mock] Charging ${}.{:02}",
                amount.0 / 100,
                amount.0 % 100
            );
            Ok(())
        }
    }

    // Console-based notification: just prints to stdout.
    // Remember Email from ex_02_dip? Same idea, different output.
    pub struct ConsoleSender;

    impl Sender for ConsoleSender {
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
}

// --- Adapter Set #2: External Services (for production) ---
// Same ports, completely different implementations.
// If we swap these and our application works with real services!
mod external_adapters {
    use crate::domain::*;
    use crate::ports::*;
    use std::collections::HashMap;

    // A "simulated" PostgreSQL adapter.
    // In real life, this would use sqlx, diesel, or similar.
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
            println!(
                "  [Postgres] INSERT INTO orders VALUES ({:?}, ...)",
                order.id
            );
            self.simulated_db.insert(order.id, order.clone());
            Ok(())
        }

        fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
            println!("  [Postgres] SELECT * FROM orders WHERE id = {:?}", id);
            Ok(self.simulated_db.get(&id).cloned())
        }
    }

    // A "simulated" Stripe adapter.
    // In real life, this would call the Stripe API.
    pub struct StripePaymentGateway;

    impl PaymentGateway for StripePaymentGateway {
        fn charge(&self, amount: Money) -> Result<(), OrderError> {
            println!(
                "  [Stripe API] POST /charges amount=${}.{:02}",
                amount.0 / 100,
                amount.0 % 100
            );
            Ok(())
        }
    }

    // A "simulated" SendGrid adapter for sending emails.
    // Same Sender trait as ConsoleSender, but talks to an email API.
    pub struct SendGridSender;

    impl Sender for SendGridSender {
        fn send(&self, order: &Order) -> Result<(), OrderError> {
            println!(
                "  [SendGrid API] Sending email: 'Order #{:?} Confirmed'",
                order.id
            );
            Ok(())
        }
    }
}

// =============================================================================
// MAIN - Putting It All Together
// =============================================================================
// Here's where the magic happens. Same OrderService, different adapters.
// We can switch from "test mode" to "production mode" just by swapping adapters.
// No changes to business logic. No changes to the application layer.
// That's the power of Hexagonal Architecture!
fn main() {
    use application::OrderService;
    use domain::{LineItem, Money, OrderId};
    use external_adapters::*;
    use in_memory_adapters::*;

    println!("=== Hexagonal Architecture Demo ===\n");

    // Some test data
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

    // --- Configuration #1: In-Memory Adapters ---
    // Perfect for testing! No external dependencies needed.
    // In ex_02_dip, this is like injecting a MockSender.
    // Here, we inject mocks for ALL our ports.
    println!("--- Configuration #1: In-Memory Adapters (Testing) ---\n");
    {
        let mut repo = InMemoryOrderRepository::new();
        let payment = MockPaymentGateway;
        let sender = ConsoleSender;

        // Same OrderService, test adapters
        let mut service = OrderService::new(&mut repo, &payment, &sender);

        match service.place_order(items.clone()) {
            Ok(order) => println!("\n  Success! Order {:?} placed.\n", order.id),
            Err(e) => println!("\n  Error: {}\n", e),
        }
    }

    // --- Configuration #2: External Services ---
    // Ready for production! Real database, real payment, real emails.
    // Notice: we didn't change a single line in OrderService or domain.
    // We just plugged in different adapters. That's DIP at scale!
    println!("--- Configuration #2: External Services (Production) ---\n");
    {
        let mut repo = PostgresOrderRepository::new();
        let payment = StripePaymentGateway;
        let sender = SendGridSender;

        // Same OrderService, production adapters
        let mut service = OrderService::new(&mut repo, &payment, &sender);

        match service.place_order(items.clone()) {
            Ok(order) => {
                println!("\n  Success! Order {:?} placed.", order.id);

                // Let's also test retrieval
                println!();
                if let Ok(Some(retrieved)) = service.get_order(order.id) {
                    println!(
                        "  Retrieved: {} items, total ${}.{:02}\n",
                        retrieved.items.len(),
                        retrieved.total.0 / 100,
                        retrieved.total.0 % 100
                    );
                }
            }
            Err(e) => println!("\n  Error: {}\n", e),
        }
    }
}

// =============================================================================
// So What Did We Learn?
// =============================================================================
//
// In dip_01, we had a problem: tight coupling.
// In ex_02_dip, we solved it with a trait and dependency injection.
// In dip_05, we scaled that solution to a real application architecture.
//
// Hexagonal Architecture is just DIP applied consistently:
// - Domain defines WHAT the business needs (via ports/traits)
// - Adapters provide HOW to fulfill those needs
// - Dependencies always point inward, toward the domain
//
// Benefits:
// 1. Testability: swap real services for mocks in tests
// 2. Flexibility: change databases or APIs without touching business logic
// 3. Clarity: each layer has a clear responsibility
// 4. Maintainability: changes are isolated to specific adapters
//
// The hexagon shape? It's just a visual metaphor. The domain is in the center,
// and adapters connect to it from the outside. Simple as that!
