// cargo run -p ex_01_dip

// =========================
// Dependency Inversion Principle - Problem
// =========================

mod bad_example {
    // Infrastructure component (low-level)
    pub struct EmailNotifier;

    impl EmailNotifier {
        pub fn send(&self, message: &str) {
            println!("Sending email: {}", message);
        }
    }

    // Business logic (high-level) DEPENDS ON infrastructure
    pub struct OrderService {
        notifier: EmailNotifier, // BAD - Direct dependency on concrete class
    }

    impl OrderService {
        pub fn new() -> Self {
            Self {
                notifier: EmailNotifier, // BAD -  Hardcoded dependency
            }
        }

        pub fn place_order(&self, order_id: u32) {
            println!("Order #{} placed", order_id);
            self.notifier
                .send(&format!("Order #{} confirmed", order_id));
        }
    }
}

fn main() {
    println!("=== Problem: Tight Coupling ===\n");
    let bad_service = bad_example::OrderService::new();
    bad_service.place_order(101);
}
