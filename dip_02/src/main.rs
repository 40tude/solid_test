// cargo run -p ex_02_dip

// =========================
// Dependency Inversion Principle - Solution
// =========================

// DOMAIN layer - defines business logic and the abstractions it needs
mod domain {
    // The business logic DEFINES what it needs
    pub trait Notifier {
        fn send(&self, message: &str);
    }

    // Business logic (high-level) DEPENDS ON abstraction
    pub struct OrderService<N: Notifier> {
        notifier: N, // Depends on trait, not concrete class
    }

    impl<N: Notifier> OrderService<N> {
        pub fn new(notifier: N) -> Self {
            Self { notifier } // Injected dependency
        }

        pub fn place_order(&self, order_id: u32) {
            println!("Order #{} placed", order_id);
            self.notifier
                .send(&format!("Order #{} confirmed", order_id));
        }
    }
}

// INFRASTRUCTURE layer - adapts to domain requirements
mod infrastructure {
    use crate::domain::Notifier; // Infrastructure depends on domain

    pub struct EmailNotifier;
    pub struct SmsNotifier;

    // Infrastructure IMPLEMENTS what the domain needs
    impl Notifier for EmailNotifier {
        fn send(&self, message: &str) {
            println!("Sending email: {}", message);
        }
    }

    impl Notifier for SmsNotifier {
        fn send(&self, message: &str) {
            println!("Sending SMS: {}", message);
        }
    }
}

fn main() {
    use domain::OrderService;
    use infrastructure::{EmailNotifier, SmsNotifier};

    println!("=== Dependency Inversion Principle ===\n");

    let email_service = OrderService::new(EmailNotifier);
    email_service.place_order(201);

    println!();

    let sms_service = OrderService::new(SmsNotifier);
    sms_service.place_order(202);
}
