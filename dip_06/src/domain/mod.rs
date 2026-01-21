// =============================================================================
// DOMAIN MODULE - The Sacred Core
// =============================================================================
//
// Welcome to the domain layer! This is the most important module in the entire
// application. Why? Because it contains the BUSINESS LOGIC (the rules that
// make your application unique and valuable).
//
// In dip_05, this code lived in a `mod domain { ... }` block inside main.rs.
// Now it has its own file. Same code, but with an important difference:
// the FILE SYSTEM now enforces our architectural rules.
//
// GOLDEN RULE OF THE DOMAIN:
// --------------------------
// Look at the imports below. What do you see?
//
//     use std::fmt;
//
// That's it. Just standard library stuff. NO imports from other modules!
// No `use crate::adapters::...`, no `use crate::ports::...`.
//
// This is the Dependency Inversion Principle in action:
// - The domain depends on NOTHING from our codebase
// - Everything else depends on the domain
//
// If you ever find yourself adding `use crate::adapters::something` here,
// STOP. You're about to violate the architecture. The domain should never
// know about databases, APIs, or any infrastructure details.

use std::fmt;

// =============================================================================
// Value Objects
// =============================================================================
// These are simple types that represent business concepts.
// They're called "value objects" because they're defined by their value,
// not by an identity. Two Money(100) are the same, interchangeable.
//
// Notice the `pub` keyword? In Rust modules, everything is private by default.
// We explicitly mark these as public so other modules can use them.
// This is different from dip_05 where everything in the same file could see
// everything else. Now we have to be intentional about our public API.

/// A unique identifier for an order.
/// Why a wrapper struct instead of just u32? Type safety!
/// You can't accidentally pass a CustomerId where an OrderId is expected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u32);

/// Represents money in cents to avoid floating-point precision issues.
/// $49.99 is stored as Money(4999).
#[derive(Debug, Clone, Copy)]
pub struct Money(pub u32);

/// A single item in an order.
#[derive(Debug, Clone)]
pub struct LineItem {
    pub name: String,
    pub price: Money,
}

// =============================================================================
// Entities
// =============================================================================
// Entities are objects with a unique identity. Two orders with the same items
// are still different orders if they have different IDs.

/// An order in our system.
/// Notice: no database IDs, no timestamps, no "created_by" fields.
/// Those are infrastructure concerns. The domain only cares about
/// what an order IS from a business perspective.
#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub items: Vec<LineItem>,
    pub total: Money,
}

// =============================================================================
// Domain Errors
// =============================================================================
// These represent things that can go wrong in our business logic.
// Notice they're business errors, not technical errors:
// - "InvalidOrder" = business rule violation
// - "PaymentFailed" = business operation failed
//
// We don't have "DatabaseConnectionError" or "HttpTimeout" here.
// Those are infrastructure errors that get translated into domain errors
// at the adapter level.

#[derive(Debug)]
pub enum OrderError {
    /// The order doesn't meet business requirements (e.g., no items)
    InvalidOrder,
    /// Payment processing failed
    PaymentFailed,
    /// Could not persist the order
    StorageFailed,
    /// Could not send notification
    NotificationFailed,
}

impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// =============================================================================
// Business Logic
// =============================================================================
// Here's where the magic happens. These are the RULES of your business.
// "An order must have at least one item": that's a business rule.
// It belongs here, not in a database constraint, not in a UI validation.
//
// Why? Because business rules should be in ONE place. If this rule changes
// (maybe we allow empty orders for some reason), we change it HERE,
// and every part of the application automatically respects the new rule.

impl Order {
    /// Creates a new order from a list of items.
    ///
    /// # Business Rules
    /// - An order must contain at least one item
    /// - The total is automatically calculated from item prices
    ///
    /// # Errors
    /// Returns `OrderError::InvalidOrder` if the items list is empty.
    pub fn new(id: OrderId, items: Vec<LineItem>) -> Result<Self, OrderError> {
        // Business rule: an order must have at least one item
        if items.is_empty() {
            return Err(OrderError::InvalidOrder);
        }

        // Calculate total - this is pure business logic
        let total = Money(items.iter().map(|item| item.price.0).sum());

        Ok(Order { id, items, total })
    }
}

// =============================================================================
// Key Takeaway
// =============================================================================
//
// This module is PURE. No side effects, no I/O, no network calls.
// You could test every function here without any mocks or setup.
// Just create values, call methods, check results.
//
// That's the power of a clean domain layer. It's simple, testable,
// and it captures the essence of your business in code.
//
// Next: check out ports/mod.rs to see how we define the boundaries!
