// =============================================================================
// EXTERNAL ADAPTERS - Simulated Production Services
// =============================================================================
//
// Welcome to the "real world" adapters! Well, simulated real world.
// These adapters pretend to talk to external services like:
// - PostgreSQL database
// - Stripe payment API
// - SendGrid email API
//
// "Wait, why 'simulated'? I thought these were production adapters!"
//
// In a real project, this is where you'd use actual libraries:
// - sqlx or diesel for PostgreSQL
// - stripe-rust for Stripe
// - sendgrid-rs for SendGrid
//
// We're simulating them because:
// 1. This is a teaching example - no need for real credentials
// 2. You can run `cargo run` without any setup
// 3. The PATTERN is what matters, not the actual API calls
//
// In your real project, you'd replace the println!() statements
// with actual library calls. The structure stays the same!

// =============================================================================
// Imports - Same Pattern as in_memory.rs
// =============================================================================

use std::collections::HashMap;

use crate::domain::{Money, Order, OrderError, OrderId};
use crate::ports::{OrderRepository, PaymentGateway, Sender};

// Same imports as in_memory.rs:
// - Standard library
// - Domain types
// - Port traits
//
// In a real production adapter, you'd also import external crates:
//
//     use sqlx::{PgPool, Row};  // for PostgreSQL
//     use stripe::{Client, CreateCharge};  // for Stripe
//     use sendgrid::v3::{Sender as SGSender, Message};  // for SendGrid
//
// Those dependencies would be listed in Cargo.toml.
// The key point: ONLY adapters have external dependencies.
// Domain, ports, and application remain pure Rust.

// =============================================================================
// PostgresOrderRepository - A "Database" Adapter
// =============================================================================
//
// In real life, this struct would hold a connection pool:
//
//     pub struct PostgresOrderRepository {
//         pool: PgPool,  // from sqlx
//     }
//
// And the methods would execute actual SQL queries.
// Here, we simulate it with a HashMap (like in_memory.rs).
// The difference? The println!() messages show what WOULD happen.

pub struct PostgresOrderRepository {
    // In reality: pool: PgPool
    // For demo: just a HashMap
    simulated_db: HashMap<OrderId, Order>,
}

impl PostgresOrderRepository {
    /// Creates a new repository.
    ///
    /// In real life, this might look like:
    /// ```ignore
    /// pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
    ///     let pool = PgPool::connect(database_url).await?;
    ///     Ok(Self { pool })
    /// }
    /// ```
    ///
    /// Notice how connection details are passed in, not hardcoded.
    /// That's another form of dependency injection!
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
    /// Saves an order to PostgreSQL.
    ///
    /// Real implementation would look like:
    /// ```ignore
    /// async fn save(&mut self, order: &Order) -> Result<(), OrderError> {
    ///     sqlx::query("INSERT INTO orders (id, total) VALUES ($1, $2)")
    ///         .bind(order.id.0)
    ///         .bind(order.total.0)
    ///         .execute(&self.pool)
    ///         .await
    ///         .map_err(|_| OrderError::StorageFailed)?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Note how database errors get converted to OrderError::StorageFailed.
    /// The application layer never sees sqlx::Error - only domain errors.
    fn save(&mut self, order: &Order) -> Result<(), OrderError> {
        // This is what a real implementation would LOG
        println!(
            "  [Postgres] INSERT INTO orders VALUES ({:?}, ...)",
            order.id
        );

        // Simulate the actual storage
        self.simulated_db.insert(order.id, order.clone());
        Ok(())
    }

    /// Retrieves an order from PostgreSQL.
    ///
    /// Real implementation:
    /// ```ignore
    /// async fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
    ///     let row = sqlx::query("SELECT * FROM orders WHERE id = $1")
    ///         .bind(id.0)
    ///         .fetch_optional(&self.pool)
    ///         .await
    ///         .map_err(|_| OrderError::StorageFailed)?;
    ///
    ///     Ok(row.map(|r| Order { ... }))
    /// }
    /// ```
    fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
        println!("  [Postgres] SELECT * FROM orders WHERE id = {:?}", id);
        Ok(self.simulated_db.get(&id).cloned())
    }
}

// =============================================================================
// StripePaymentGateway - A Payment API Adapter
// =============================================================================
//
// Stripe is a popular payment processor. Their Rust SDK would be used here.
// The adapter translates between our domain (Money) and Stripe's API.

pub struct StripePaymentGateway;

// In real life, you might have:
//
//     pub struct StripePaymentGateway {
//         client: stripe::Client,
//         secret_key: String,
//     }
//
//     impl StripePaymentGateway {
//         pub fn new(secret_key: &str) -> Self {
//             Self {
//                 client: stripe::Client::new(secret_key),
//                 secret_key: secret_key.to_string(),
//             }
//         }
//     }

impl PaymentGateway for StripePaymentGateway {
    /// Charges a customer via Stripe.
    ///
    /// Real implementation:
    /// ```ignore
    /// async fn charge(&self, amount: Money) -> Result<(), OrderError> {
    ///     let charge = CreateCharge {
    ///         amount: amount.0 as i64,  // Stripe uses cents too!
    ///         currency: "usd",
    ///         source: "tok_visa",  // In reality, from frontend
    ///         ..Default::default()
    ///     };
    ///
    ///     self.client
    ///         .charges()
    ///         .create(charge)
    ///         .await
    ///         .map_err(|_| OrderError::PaymentFailed)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Notice how Stripe errors become OrderError::PaymentFailed.
    /// The application layer doesn't know about stripe::Error.
    fn charge(&self, amount: Money) -> Result<(), OrderError> {
        println!(
            "  [Stripe API] POST /charges amount=${}.{:02}",
            amount.0 / 100,
            amount.0 % 100
        );
        Ok(())
    }
}

// =============================================================================
// SendGridSender - An Email API Adapter
// =============================================================================
//
// SendGrid is a popular email service. This adapter would use their API
// to send order confirmation emails.
//
// Remember Email from dip_02? This is its production-ready cousin!

pub struct SendGridSender;

// In real life:
//
//     pub struct SendGridSender {
//         api_key: String,
//         from_email: String,
//     }
//
//     impl SendGridSender {
//         pub fn new(api_key: &str, from_email: &str) -> Self {
//             Self {
//                 api_key: api_key.to_string(),
//                 from_email: from_email.to_string(),
//             }
//         }
//     }

impl Sender for SendGridSender {
    /// Sends an order confirmation email via SendGrid.
    ///
    /// Real implementation:
    /// ```ignore
    /// async fn send(&self, order: &Order) -> Result<(), OrderError> {
    ///     let message = Message::new()
    ///         .set_from(self.from_email.clone())
    ///         .set_subject(format!("Order #{:?} Confirmed", order.id))
    ///         .add_content(/* HTML template */);
    ///
    ///     sendgrid::send(&self.api_key, &message)
    ///         .await
    ///         .map_err(|_| OrderError::NotificationFailed)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn send(&self, order: &Order) -> Result<(), OrderError> {
        println!(
            "  [SendGrid API] Sending email: 'Order #{:?} Confirmed'",
            order.id
        );
        Ok(())
    }
}

// =============================================================================
// Error Translation - A Critical Responsibility
// =============================================================================
//
// Notice something in all the "real implementation" examples above?
// They all convert external errors to domain errors:
//
//     .map_err(|_| OrderError::StorageFailed)
//     .map_err(|_| OrderError::PaymentFailed)
//     .map_err(|_| OrderError::NotificationFailed)
//
// This is CRUCIAL. The adapter's job is to:
// 1. Call the external service
// 2. Translate the result into domain terms
//
// The application layer should NEVER see:
// - sqlx::Error
// - stripe::Error
// - reqwest::Error
//
// Only OrderError. This keeps the domain and application layers
// completely independent of infrastructure details.

// =============================================================================
// Async Considerations
// =============================================================================
//
// You might have noticed the "async" in the example implementations.
// Real database and HTTP calls are asynchronous in Rust.
//
// To support async, you'd change the port traits:
//
//     #[async_trait]
//     pub trait OrderRepository {
//         async fn save(&mut self, order: &Order) -> Result<(), OrderError>;
//         async fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError>;
//     }
//
// The async_trait crate makes this work with trait objects.
// Our sync version is simpler for learning, but real apps often need async.

// =============================================================================
// Key Takeaway
// =============================================================================
//
// These adapters show the PATTERN for production code:
//
// 1. HOLD external clients (connection pools, API clients)
// 2. IMPLEMENT port traits using those clients
// 3. TRANSLATE between external types and domain types
// 4. CONVERT errors to domain errors
//
// The println!() statements are placeholders for real API calls.
// The structure is exactly what you'd use in production.
//
// This is the outer ring of hexagonal architecture:
// - The ONLY place with external crate dependencies
// - The ONLY place that knows about SQL, HTTP, etc.
// - The ONLY place that changes when you switch technologies
//
// Everything else (domain, ports, application) stays pure and stable!

// =============================================================================
// Congratulations!
// =============================================================================
//
// You've completed the tour of dip_06!
//
// Let's recap the journey from dip_01 to dip_06:
//
// dip_01: The problem - tight coupling, domain depends on infrastructure
// dip_02: The solution - invert dependencies with a trait (Sender)
// dip_03: Multiple adapters - Email, SMS, Owl (same trait, different impls)
// dip_04: Testing - MockSender makes unit tests easy
// dip_05: Hexagonal Architecture - multiple ports, layered structure
// dip_06: Modular organization - file system enforces architecture
//
// You now understand DIP at every level: concept, pattern, and structure.
// Go build something amazing with it!
