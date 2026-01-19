// cargo test -p ex_03_dip
// cargo run -p ex_03_dip

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

// =========================
// TESTING - The real benefit of DIP
// =========================

#[cfg(test)]
mod tests {
    use super::domain::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    // Mock notifier for testing - no real infrastructure needed!
    struct MockNotifier {
        messages: Rc<RefCell<Vec<String>>>, // Shared ownership for verification
    }

    impl MockNotifier {
        fn new() -> (Self, Rc<RefCell<Vec<String>>>) {
            let messages = Rc::new(RefCell::new(Vec::new()));
            (
                Self {
                    messages: Rc::clone(&messages),
                },
                messages,
            )
        }
    }

    // Implement the domain's trait - that's all we need!
    impl Notifier for MockNotifier {
        fn send(&self, message: &str) {
            self.messages.borrow_mut().push(message.to_string());
        }
    }

    #[test]
    fn test_order_service_sends_notification() {
        // Arrange: Create service with mock
        let (mock, messages) = MockNotifier::new();
        let service = OrderService::new(mock);

        // Act: Execute business logic
        service.place_order(42);

        // Assert: Verify the notification was sent
        let msgs = messages.borrow();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0], "Order #42 confirmed");
    }

    #[test]
    fn test_multiple_orders() {
        // Arrange
        let (mock, messages) = MockNotifier::new();
        let service = OrderService::new(mock);

        // Act: Place multiple orders
        service.place_order(100);
        service.place_order(101);
        service.place_order(102);

        // Assert: All notifications were sent
        let msgs = messages.borrow();
        assert_eq!(msgs.len(), 3);
        assert!(msgs[0].contains("Order #100"));
        assert!(msgs[1].contains("Order #101"));
        assert!(msgs[2].contains("Order #102"));
    }

    #[test]
    fn test_notification_format() {
        // Arrange
        let (mock, messages) = MockNotifier::new();
        let service = OrderService::new(mock);

        // Act
        service.place_order(999);

        // Assert: Verify exact message format
        let msgs = messages.borrow();
        assert_eq!(msgs[0], "Order #999 confirmed");
    }

    // We could also test error cases, edge cases, etc.
    // All without touching any real infrastructure!
}
