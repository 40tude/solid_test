// cargo run -p ex_02_dip

// Alright! Now we're going to fix the problem we saw in ex00.
// The magic word? Dependency Inversion.
// Instead of business logic depending on infrastructure,
// we'll make infrastructure depend on business logic. Let's see how!

// Look at main() now. Something changed.
// We're injecting the Email service into OrderService from the outside.
// The caller decides which notification system to use!
// Want SMS tomorrow? Just pass an Sms struct instead. No changes to OrderService needed.
fn main() {
    use domain::OrderService;
    use email::Email;

    let email_service = OrderService::new(Email);
    email_service.place_order(101);
}

// This is where the magic happens.
// Notice: domain doesn't import anything from email anymore!
// The dependency arrow has been reversed.
mod domain {

    // Here's the key: we define a trait called Sender.
    // This is our "contract" - any notification system must follow this contract.
    // The business logic says: "I don't care HOW you send messages,
    // just give me something that has a send() method."
    pub trait Sender {
        fn send(&self, message: &str);
    }

    // Now OrderService is generic over S, where S must implement Sender.
    // It doesn't know if S is Email, SMS, a carrier pigeon... and it doesn't care!
    // It just knows that S can send messages. That's all it needs.
    pub struct OrderService<S: Sender> {
        sender: S,
    }

    impl<S: Sender> OrderService<S> {
        // The dependency is now injected from outside.
        // We receive the sender as a parameter - we don't create it ourselves.
        // This is called "Dependency Injection", and it goes hand in hand with DIP.
        pub fn new(sender: S) -> Self {
            Self { sender }
        }

        pub fn place_order(&self, order_id: u32) {
            println!("Order #{} placed", order_id);

            // See? We call self.sender.send() without knowing what's behind it.
            // If Email renames its internal method? We don't care!
            // As long as it implements our Sender trait, we're good.
            self.sender.send(&format!("Order #{} confirmed", order_id));
        }
    }
}

// Now look at this: email imports from domain, not the other way around!
// The dependency arrow is reversed. Infrastructure depends on business logic.
// That's exactly what "Dependency Inversion" means.
mod email {
    use crate::domain::Sender;

    pub struct Email;

    // Email adapts itself to the contract defined by domain.
    // It implements the Sender trait, promising to provide a send() method.
    // Internally, it could call dispatch(), post_message(), or whatever it wants.
    // The domain doesn't care about those details.
    impl Sender for Email {
        fn send(&self, message: &str) {
            println!("Sending by email: {}", message);
        }
    }
}

// So what did we gain?
// 1. OrderService is now testable - we can inject a mock Sender
// 2. We can swap Email for SMS without touching business logic
// 3. If Email changes its internals, domain is protected
// 4. The architecture is cleaner: high-level doesn't depend on low-level
//
// Next: see ex02 for adding SMS and other notification services!
