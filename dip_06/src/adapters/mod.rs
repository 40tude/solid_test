// =============================================================================
// ADAPTERS MODULE - The Gateway to the Outside World
// =============================================================================
//
// Welcome to the outermost layer of our hexagonal architecture!
// This is where abstractions meet reality. Where traits become implementations.
// Where "I need to save an order" becomes actual SQL queries.
//
// In dip_02, our adapter was simple: the Email struct implementing Sender.
// In dip_05, we had two sets: in_memory_adapters and external_adapters.
// Now in dip_06, they're organized into proper sub-modules.
//
// THE ADAPTERS FOLDER STRUCTURE:
// ------------------------------
//   adapters/
//   ├── mod.rs        <- you are here! (this file)
//   ├── in_memory.rs  <- test/development implementations
//   └── external.rs   <- production implementations
//
// This is a common pattern in Rust: a folder with mod.rs acts like
// a module that can contain sub-modules. It's perfect for grouping
// related implementations.

// =============================================================================
// Sub-Module Declarations
// =============================================================================
// These two lines are all the "code" in this file. But they're important!
//
// `pub mod external` means:
//   1. Look for a file named external.rs in this directory
//   2. Load it as a sub-module named "external"
//   3. Make it PUBLIC (pub) so other modules can use it
//
// Without `pub`, these modules would be private to the adapters module.
// main.rs wouldn't be able to import PostgresOrderRepository!

pub mod external;
pub mod in_memory;

// =============================================================================
// Why This File Exists
// =============================================================================
//
// You might wonder: "Why have this nearly-empty file?"
// Good question! It serves several purposes:
//
// 1. NAMESPACE ORGANIZATION
//    Instead of: `use crate::external_adapters::PostgresOrderRepository`
//    We get:     `use crate::adapters::external::PostgresOrderRepository`
//
//    The path tells a story: "This is an adapter, specifically an external one."
//
// 2. CONTROLLED VISIBILITY
//    We choose what's public. If we wanted to hide certain adapters,
//    we could make them private or use `pub(crate)` for internal-only access.
//
// 3. RE-EXPORTS (optional)
//    We COULD re-export items here for convenience:
//
//        pub use external::PostgresOrderRepository;
//        pub use in_memory::InMemoryOrderRepository;
//
//    Then users could write: `use crate::adapters::PostgresOrderRepository`
//    without the extra `external::` in the path.
//
//    We didn't do that here because explicit paths make the architecture clearer.
//    But it's a valid choice for larger projects where convenience matters.
//
// 4. FUTURE EXTENSION POINT
//    Need to add a new adapter category? Just add another line:
//
//        pub mod external;
//        pub mod in_memory;
//        pub mod grpc;        // new!
//        pub mod graphql;     // new!
//
//    The structure grows naturally.

// =============================================================================
// Adapters and the Dependency Rule
// =============================================================================
//
// Here's a CRITICAL architectural point. Look at the imports in external.rs
// and in_memory.rs:
//
//     use crate::domain::{...};
//     use crate::ports::{...};
//
// Adapters import FROM domain and ports. Never the reverse!
//
// This means:
// - domain/ doesn't know adapters exist
// - ports/ doesn't know adapters exist
// - application/ doesn't know adapters exist
//
// Only main.rs knows about adapters, and that's where it wires everything up.
//
// WHY THIS MATTERS:
// -----------------
// Imagine you need to switch from PostgreSQL to MongoDB. What files change?
//
//     adapters/external.rs  <- yes, write new MongoOrderRepository
//     main.rs               <- yes, import and wire it up
//     domain/mod.rs         <- NO CHANGES
//     ports/mod.rs          <- NO CHANGES
//     application/mod.rs    <- NO CHANGES
//
// The core of your application is PROTECTED from infrastructure changes.
// That's the whole point of DIP and hexagonal architecture!

// =============================================================================
// Adapter Isolation
// =============================================================================
//
// Here's something subtle but important: adapters are ISOLATED from each other.
//
// Look at in_memory.rs. It imports from domain and ports.
// Look at external.rs. It imports from domain and ports.
//
// But external.rs doesn't import from in_memory.rs, and vice versa!
// Each adapter set is independent. This prevents coupling between adapters.
//
// Why does this matter? Imagine InMemoryOrderRepository had a bug.
// You fix it. What code could possibly be affected?
// - Code that uses InMemoryOrderRepository (tests, dev environment)
// - NOT code that uses PostgresOrderRepository (production)
//
// The blast radius of changes is minimized.

// =============================================================================
// Key Takeaway
// =============================================================================
//
// The adapters module is the OUTER RING of your architecture:
//
//         ╭─────────────────────────────────────╮
//         │            ADAPTERS                 │
//         │   ┌─────────────────────────────┐   │
//         │   │       APPLICATION           │   │
//         │   │   ┌───────────────────┐     │   │
//         │   │   │      DOMAIN       │     │   │
//         │   │   │                   │     │   │
//         │   │   └───────────────────┘     │   │
//         │   │         PORTS               │   │
//         │   └─────────────────────────────┘   │
//         ╰─────────────────────────────────────╯
//
// Adapters are the ONLY place where external dependencies live:
// - Database drivers (sqlx, diesel)
// - HTTP clients (reqwest)
// - Message queues (lapin, rdkafka)
// - External APIs (stripe-rust, aws-sdk)
//
// The rest of your codebase is pure Rust with no external runtime dependencies.
//
// Next: dive into in_memory.rs and external.rs to see actual implementations!
