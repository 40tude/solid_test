// cargo run -p ex_05_dip

//! Hexagonal Architecture Demo - aka Ports & Adapters
//!
//! This example demonstrates the Dependency Inversion Principle (DIP)
//! through hexagonal architecture, showing how to decouple business logic
//! from infrastructure concerns.

mod adapters;
mod application;
mod domain;
mod ports;

use adapters::external::{PostgresOrderRepository, SendGridNotificationService, StripePaymentGateway};
use adapters::in_memory::{ConsoleNotificationService, InMemoryOrderRepository, MockPaymentGateway};
use application::OrderService;
use domain::{LineItem, Money};

fn main() {
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
