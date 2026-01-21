// cargo run -p ex_01_dip

// Bonjour. In this first example, we're going to see a common mistake.
// It works, sure, but there's a hidden problem lurking in the code.
// Let's discover it together.

// Here in main(), everything looks fine, right?
// We create an OrderService, we place an order... simple and clean.
// The caller doesn't need to know how notifications are sent.
// So far so good!
fn main() {
    use order::OrderService;

    let order_service = OrderService::new();
    order_service.place_order(101);
}

// Now let's look behind the curtain...
// This is where things get interesting.
mod order {

    // Oops! See this `use` statement?
    // Our business logic (OrderService) is importing a concrete type from infrastructure.
    // That's our first red flag: the high-level module knows about the low-level module.
    use crate::email::Sender;

    pub struct OrderService {
        // And here, we're storing an email::Sender directly.
        // What if tomorrow we want to send SMS instead? Or push notifications?
        // We'd have to come here and modify this struct. That's not great.
        email_sender: Sender,
    }

    impl OrderService {
        pub fn new() -> Self {
            Self {
                // The dependency is hardcoded right here.
                // OrderService decides by itself that it will use email::Sender.
                // No flexibility, no way to swap implementations.
                email_sender: Sender,
            }
        }

        pub fn place_order(&self, order_id: u32) {
            println!("Order #{} placed", order_id);

            // And here's another problem: we're calling .send() directly.
            // What if the email team decides to rename it to .dispatch()?
            // Our business logic breaks! And that shouldn't happen.
            // Business rules shouldn't break because of infrastructure changes.
            self.email_sender
                .send(&format!("Order #{} confirmed", order_id));
        }
    }
}

// This is our infrastructure layer - the "low-level" stuff.
// It handles the technical details of sending emails.
mod email {
    pub struct Sender;

    impl Sender {
        pub fn send(&self, message: &str) {
            println!("Sending email: {}", message);
        }

        // Imagine the email team wants to rename send() to dispatch()...
        // Go ahead, uncomment this and comment out send() above.
        // You'll see: the code in order module breaks!
        // That's the problem we'll solve in ex01.
        //
        // pub fn dispatch(&self, message: &str) {
        //     println!("Sending email: {}", message);
        // }
    }
}
