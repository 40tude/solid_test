// cargo run -p ex_06_dip

// =============================================================================
// Welcome to dip_06! This is the same code as dip_05, but split into modules.
// =============================================================================
//
// "Wait, why bother splitting files if the code is the same?"
//
// Great question! In dip_05, everything lived in one big main.rs file.
// That worked fine for learning, but real projects need better organization.
// Splitting into modules gives us:
//
// 1. ENFORCED BOUNDARIES: In dip_05, nothing stopped you from importing
//    email stuff directly into domain. Here, the file system itself
//    creates barriers. You physically CAN'T mess up the architecture
//    without deliberately changing the module structure.
//
// 2. TEAM SCALABILITY: Different developers can work on different files
//    without constant merge conflicts. The domain team works in domain/,
//    the infrastructure team works in adapters/.
//
// 3. COMPILE-TIME GUARANTEES: Rust's module system enforces visibility.
//    If something isn't marked `pub`, other modules can't see it.
//    The compiler becomes your architecture guardian!
//
// Let's see how it all fits together.

// =============================================================================
// Module Declarations
// =============================================================================
// These four lines are the backbone of our architecture.
// Each `mod` statement tells Rust: "Look for a folder with this name,
// and load the mod.rs file inside it."
//
// Notice the order doesn't matter for declarations, but it helps readability
// to list them in dependency order: domain first (depends on nothing),
// then ports (depends on domain), then application (depends on both),
// and finally adapters (depends on domain + ports).

mod adapters;
mod application;
mod domain;
mod ports;

// =============================================================================
// Imports - This Is Where It Gets Interesting!
// =============================================================================
// Look at these import paths. They tell a story about our architecture.
//
// adapters::external::PostgresOrderRepository
//          ^^^^^^^^ ^^^^^^^^^^^^^^^^^^^^^^
//          |        |
//          |        The actual struct
//          |
//          Sub-module inside adapters/
//
// In dip_05, we had: `use external_adapters::PostgresOrderRepository`
// Now we have:       `use adapters::external::PostgresOrderRepository`
//
// The path reflects the folder structure:
//   src/
//   ├── adapters/
//   │   ├── mod.rs          <- declares `pub mod external;`
//   │   ├── external.rs     <- contains PostgresOrderRepository
//   │   └── in_memory.rs    <- contains InMemoryOrderRepository
//   ├── application/
//   │   └── mod.rs          <- contains OrderService
//   ├── domain/
//   │   └── mod.rs          <- contains Order, Money, etc.
//   ├── ports/
//   │   └── mod.rs          <- contains traits (Sender, etc.)
//   └── main.rs             <- you are here!

use adapters::external::{PostgresOrderRepository, SendGridSender, StripePaymentGateway};
use adapters::in_memory::{ConsoleSender, InMemoryOrderRepository, MockPaymentGateway};
use application::OrderService;
use domain::{LineItem, Money};

// Notice what we DON'T import: anything from `ports`.
// Why? Because main.rs doesn't need to know about the traits!
// It just creates concrete adapters and passes them to OrderService.
// The generic constraints are checked at compile time, but we don't
// need to spell them out here. That's the beauty of type inference.

// =============================================================================
// Main Function - Same as dip_05, Nothing Changed Here!
// =============================================================================
// This is the payoff: the actual usage code is IDENTICAL to dip_05.
// We reorganized the entire codebase, and main() didn't notice.
// That's a sign of good architecture (internal changes don't ripple outward).

fn main() {
    println!("=== Hexagonal Architecture Demo (Modular) ===\n");

    // Test data: same as ex_05_dip
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

    // -------------------------------------------------------------------------
    // Configuration #1: In-Memory Adapters (Testing)
    // -------------------------------------------------------------------------
    // Perfect for unit tests. No database, no network, no external services.
    // Everything runs in memory, fast and deterministic.
    println!("--- Configuration #1: In-Memory Adapters (Testing) ---\n");
    {
        let mut repo = InMemoryOrderRepository::new();
        let payment = MockPaymentGateway;
        let sender = ConsoleSender;

        let mut service = OrderService::new(&mut repo, &payment, &sender);

        match service.place_order(items.clone()) {
            Ok(order) => println!("\nOrder placed successfully: {:?}\n", order.id),
            Err(e) => println!("\nError: {}\n", e),
        }
    }

    // -------------------------------------------------------------------------
    // Configuration #2: External Services (Production)
    // -------------------------------------------------------------------------
    // Same OrderService, completely different adapters.
    // In a real app, you'd choose the configuration based on environment
    // variables or a config file. The point is: OrderService doesn't care!
    println!("--- Configuration #2: External Services (Production) ---\n");
    {
        let mut repo = PostgresOrderRepository::new();
        let payment = StripePaymentGateway;
        let sender = SendGridSender;

        let mut service = OrderService::new(&mut repo, &payment, &sender);

        match service.place_order(items.clone()) {
            Ok(order) => {
                println!("\nOrder placed successfully: {:?}", order.id);

                // Let's also test retrieval
                println!();
                if let Ok(Some(retrieved)) = service.get_order(order.id) {
                    println!(
                        "Retrieved order: {} items, total ${}.{:02}\n",
                        retrieved.items.len(),
                        retrieved.total.0 / 100,
                        retrieved.total.0 % 100
                    );
                }
            }
            Err(e) => println!("\nError: {}\n", e),
        }
    }
}

// =============================================================================
// What's Next?
// =============================================================================
//
// Now dive into each module to see how they're organized:
//
// 1. domain/mod.rs      : The heart: pure business logic, no dependencies
// 2. ports/mod.rs       : The contracts: traits that define boundaries
// 3. application/mod.rs : The orchestrator: coordinates domain + ports
// 4. adapters/mod.rs    : The gateway to infrastructure implementations
//    ├── in_memory.rs   : Test/dev implementations
//    └── external.rs    : Production implementations
//
// Each file has its own comments explaining its role. Enjoy the tour!
