// =============================================================================
// APPLICATION MODULE - The Orchestrator
// =============================================================================
//
// In dip_02, OrderService lived in the domain module. It was simple back then:
// just one Sender trait and one method. But as applications grow, we need
// to separate "what things ARE" (domain) from "what HAPPENS" (application).
//
// Think of it like a restaurant:
// - Domain = recipes, ingredients, cooking techniques (the knowledge)
// - Application = the head chef orchestrating a dinner service (the coordination)
//
// The chef doesn't invent new recipes during service. They follow recipes
// (domain rules) and coordinate the kitchen (call adapters through ports).
//
// IN DIP_05 VS DIP_06:
// --------------------
// Same code, but now in its own file. This makes the role crystal clear:
// application/mod.rs is WHERE THINGS HAPPEN. It's the entry point for
// use cases, the coordinator that ties everything together.

// =============================================================================
// Imports - Notice the Pattern
// =============================================================================
// Let's look at what we import and from where:

use crate::domain::{LineItem, Order, OrderError, OrderId};
use crate::ports::{OrderRepository, PaymentGateway, Sender};

// Two sources:
// 1. crate::domain - the business entities and errors
// 2. crate::ports  - the traits (abstractions) we depend on
//
// CRITICALLY: no `use crate::adapters::...` here!
//
// The application layer doesn't know (or care) whether we're using
// PostgreSQL or an in-memory HashMap. It just knows it has something
// that implements OrderRepository. That's the whole point of DIP!
//
// This is what makes your code testable: in tests, you can inject mock
// implementations. In production, you inject real ones. Same OrderService,
// different adapters.

// =============================================================================
// The Order Service - Your Use Case Handler
// =============================================================================
// This struct is generic over THREE type parameters: R, P, and N.
// Each one is constrained by a trait from the ports module.
//
// In dip_02, we had: OrderService<S: Sender>
// Now we have:       OrderService<R: OrderRepository, P: PaymentGateway, N: Sender>
//
// Same pattern, just scaled up. More dependencies, more flexibility.

pub struct OrderService<R, P, N>
where
    R: OrderRepository,
    P: PaymentGateway,
    N: Sender,
{
    // These fields hold our adapters. But we only know them by their traits!
    // `repository` could be PostgresOrderRepository or InMemoryOrderRepository.
    // We don't know, and we don't need to know. That's abstraction at work.
    repository: R,
    payment: P,
    sender: N,

    // This is application state, not business logic.
    // In a real app, IDs would come from the database or a UUID generator.
    next_id: u32,
}

// =============================================================================
// Implementation - Where Orchestration Happens
// =============================================================================

impl<R, P, N> OrderService<R, P, N>
where
    R: OrderRepository,
    P: PaymentGateway,
    N: Sender,
{
    /// Creates a new OrderService with the given adapters.
    ///
    /// This is Dependency Injection in action! The caller (main.rs) decides
    /// which concrete implementations to use. OrderService just accepts
    /// anything that implements the required traits.
    ///
    /// # Why This Matters
    /// - Testing: pass mock adapters, no real database needed
    /// - Flexibility: swap PostgreSQL for MongoDB without changing this code
    /// - Clarity: dependencies are explicit, not hidden in the implementation
    pub fn new(repository: R, payment: P, sender: N) -> Self {
        Self {
            repository,
            payment,
            sender,
            next_id: 1,
        }
    }

    /// Places a new order - this is the main "use case".
    ///
    /// Look at what this method does:
    /// 1. Generate an ID (application concern)
    /// 2. Create the Order (delegates to domain)
    /// 3. Charge payment (calls port -> adapter)
    /// 4. Save order (calls port -> adapter)
    /// 5. Send notification (calls port -> adapter)
    ///
    /// Notice the ORDER of operations matters here. That's orchestration!
    /// We charge before saving because we don't want to save an order
    /// that wasn't paid for. These decisions live in the application layer.
    pub fn place_order(&mut self, items: Vec<LineItem>) -> Result<Order, OrderError> {
        // Step 1: Generate an ID (application layer responsibility)
        let order_id = OrderId(self.next_id);
        self.next_id += 1;

        // Step 2: Create the order using domain logic
        // Order::new() enforces business rules (like "must have items")
        let order = Order::new(order_id, items)?;

        // Steps 3-5: Orchestrate the external operations
        // Each of these calls goes through a port (trait) to an adapter.
        // We don't know if we're calling Stripe or a mock. We don't care!
        self.payment.charge(order.total)?;  // Charge first!
        self.repository.save(&order)?;       // Then persist
        self.sender.send(&order)?;           // Finally notify

        Ok(order)
    }

    /// Retrieves an order by ID.
    ///
    /// A simpler use case: just delegate to the repository.
    /// No orchestration needed here, just a passthrough.
    pub fn get_order(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
        self.repository.find(id)
    }
}

// =============================================================================
// Why Not Put This in Domain?
// =============================================================================
//
// You might think: "OrderService deals with orders, so it belongs in domain!"
// But look at what it does:
//
// - It calls external services (payment, storage, notifications)
// - It coordinates operations in a specific order
// - It manages application state (next_id)
//
// None of that is business LOGIC. It's application WORKFLOW.
// The business rules live in Order::new() - "an order must have items".
// The workflow lives here - "charge, then save, then notify".
//
// Keeping them separate means:
// - Domain stays pure and testable without any mocks
// - Application can be tested with mock adapters
// - Changes to workflow don't touch domain code
// - Changes to business rules don't touch workflow code

// =============================================================================
// Key Takeaway
// =============================================================================
//
// The application layer is the GLUE between your domain and the outside world.
// It knows about domain entities AND port traits, but not about adapters.
//
// This is the middle layer of the hexagonal architecture:
//
//     [Adapters] ---> [Application] ---> [Domain]
//                           |
//                           v
//                       [Ports]
//
// The application coordinates; it doesn't implement infrastructure details.
//
// Next: check out adapters/mod.rs to see the concrete implementations!
