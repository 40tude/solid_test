// cargo test -p ex_04_dip
// cargo run -p ex_04_dip
// Add testing

fn main() {
    use domain::OrderService;
    use email::Email;
    use owl::Owl;
    use sms::Sms;

    let email_service = OrderService::new(Email);
    email_service.place_order(101);
    println!();
    let sms_service = OrderService::new(Sms);
    sms_service.place_order(42);
    println!();
    let owl_service = OrderService::new(Owl);
    owl_service.place_order(13);
}

mod domain {
    pub trait Sender {
        fn send(&self, message: &str);
    }
    pub struct OrderService<S: Sender> {
        sender: S,
    }

    impl<S: Sender> OrderService<S> {
        pub fn new(sender: S) -> Self {
            Self { sender }
        }

        pub fn place_order(&self, order_id: u32) {
            println!("Order #{} placed", order_id);

            self.sender.send(&format!("Order #{} confirmed", order_id));
        }
    }
}

mod email {
    use crate::domain::Sender;

    pub struct Email;

    impl Sender for Email {
        fn send(&self, message: &str) {
            println!("Sending by email: {}", message);
        }
    }
}

mod sms {
    use crate::domain::Sender;
    pub struct Sms;

    impl Sender for Sms {
        fn send(&self, message: &str) {
            println!("Sending by sms: {}", message);
        }
    }
}

mod owl {
    use crate::domain::Sender;
    pub struct Owl;

    impl Sender for Owl {
        fn send(&self, message: &str) {
            println!("Sending by 🦉: {}", message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::domain::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    // Mock sender for testing - no real infrastructure needed!
    struct MockSender {
        messages: Rc<RefCell<Vec<String>>>, // Shared ownership for verification
    }

    impl MockSender {
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
    impl Sender for MockSender {
        fn send(&self, message: &str) {
            self.messages.borrow_mut().push(message.to_string());
        }
    }

    #[test]
    fn test_order_service_sends_notification() {
        // Arrange: Create service with mock
        let (mock, messages) = MockSender::new();
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
        let (mock, messages) = MockSender::new();
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
        let (mock, messages) = MockSender::new();
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
