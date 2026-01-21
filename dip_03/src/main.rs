// cargo run -p ex_03_dip
// Add sms and owl services 🦉

fn main() {
    use domain::OrderService;
    use email::Email;
    // + 2 lines here
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

// No change here
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

// No change here
mod email {
    use crate::domain::Sender;

    pub struct Email;

    impl Sender for Email {
        fn send(&self, message: &str) {
            println!("Sending by email: {}", message);
        }
    }
}

// +1 service here
mod sms {
    use crate::domain::Sender;
    pub struct Sms;

    impl Sender for Sms {
        fn send(&self, message: &str) {
            println!("Sending by sms: {}", message);
        }
    }
}

// +1 service here
mod owl {
    use crate::domain::Sender;
    pub struct Owl;

    impl Sender for Owl {
        fn send(&self, message: &str) {
            println!("Sending by 🦉: {}", message);
        }
    }
}
