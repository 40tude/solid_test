// =============================================================================
// PORTS MODULE - The Boundaries of Your Domain
// =============================================================================
//
// Remember the Sender trait from dip_02? That was our first "port".
// A port is just a trait that defines what the domain NEEDS from the
// outside world, without specifying HOW that need will be fulfilled.
//
// In dip_05, ports were a separate `mod ports { ... }` block, but still
// in the same file. Now they live in their own module.
//
// WHY SEPARATE PORTS FROM DOMAIN?
// --------------------------------
// You might wonder: "Why not put these traits in domain/mod.rs?"
// Good question! There are two schools of thought:
//
// 1. Ports IN domain: The domain owns its contracts (common in DDD)
// 2. Ports SEPARATE: Clear visual distinction between "what is" and "what's needed"
//
// We chose #2 because it makes the architecture more obvious:
// - domain/ = business entities and rules (pure data + logic)
// - ports/  = interfaces to the outside world (traits)
//
// Both approaches are valid. The key is consistency within your project.

// =============================================================================
// The Import That Matters
// =============================================================================
// Look at this import carefully:

use crate::domain::{Money, Order, OrderError, OrderId};

// We import FROM domain. That's the correct dependency direction!
// Ports know about domain types because they need to speak the domain's
// language. But domain doesn't know about ports - check domain/mod.rs,
// you won't find any `use crate::ports::...` there.
//
// This is Dependency Inversion in action:
//
//     domain  <----  ports  <----  adapters
//        ^             ^              |
//        |             |              |
//        +-------------+--------------+
//              Dependencies flow INWARD
//
// The domain is at the center, depending on nothing.
// Ports point inward to domain.
// Adapters point inward to ports (and transitively to domain).

// =============================================================================
// Output Ports (aka "Driven Ports")
// =============================================================================
// These traits define what the application needs from infrastructure.
// They're called "output" or "driven" ports because they represent
// actions that the application wants to PERFORM on the outside world:
// - "I need to SAVE an order somewhere"
// - "I need to CHARGE a payment"
// - "I need to SEND a notification"
//
// The application calls OUT through these ports.

/// Port for persisting and retrieving orders.
///
/// Notice the method signatures use domain types (Order, OrderId, OrderError).
/// The trait doesn't mention SQL, MongoDB, or any storage technology.
/// That's an implementation detail for adapters to decide.
pub trait OrderRepository {
    /// Saves an order to persistent storage.
    fn save(&mut self, order: &Order) -> Result<(), OrderError>;

    /// Retrieves an order by its ID, if it exists.
    fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError>;
}

/// Port for processing payments.
///
/// Why Money and not f64 or Decimal? Because Money is a domain concept.
/// The port speaks the domain's language.
pub trait PaymentGateway {
    /// Charges the specified amount.
    fn charge(&self, amount: Money) -> Result<(), OrderError>;
}

/// Port for sending notifications to customers.
///
/// Hey, look familiar? This is our old friend Sender from dip_02!
/// Same concept: "I need to notify someone about an order."
/// Could be email, SMS, push notification, carrier pigeon...
pub trait Sender {
    /// Sends a notification about an order.
    fn send(&self, order: &Order) -> Result<(), OrderError>;
}

// =============================================================================
// A Note on Input Ports
// =============================================================================
//
// You might read about "input ports" or "driving ports" in hexagonal
// architecture literature. Those represent entry points INTO the application
// (like HTTP handlers or CLI commands).
//
// In our example, we don't have explicit input ports because:
// 1. main() directly calls OrderService - it's simple enough
// 2. We're focusing on the DIP story, not full hexagonal architecture
//
// In a real application, you might define:
//
//     pub trait OrderUseCase {
//         fn place_order(&mut self, items: Vec<LineItem>) -> Result<Order, OrderError>;
//         fn get_order(&self, id: OrderId) -> Result<Option<Order>, OrderError>;
//     }
//
// And have OrderService implement it. Your HTTP handler would then depend
// on the trait, not the concrete service. That's full DIP for input AND output.

// =============================================================================
// Key Takeaway
// =============================================================================
//
// Ports are CONTRACTS. They define:
// - WHAT the application needs (save orders, charge payments, send notifications)
// - NOT HOW those needs are fulfilled (SQL vs NoSQL, Stripe vs PayPal, etc.)
//
// This separation is what makes your application flexible. Need to switch
// from PostgreSQL to MongoDB? Write a new adapter. The ports don't change.
// Need to test without real services? Use mock adapters. Ports don't care.
//
// Next: check out application/mod.rs to see who USES these ports!
