//Define the Strategy Pattern for Payment Methods
trait PaymentStrategy {
    fn pay(&self, amount: f64);
}

struct CreditCardPayment {
    name: String,
    card_number: String,
}

impl CreditCardPayment {
    fn new(name: &str, card_number: &str) -> Self {
        CreditCardPayment {
            name: name.to_string(),
            card_number: card_number.to_string(),
        }
    }
}

impl PaymentStrategy for CreditCardPayment {
    fn pay(&self, amount: f64) {
        println!(
            "Paying ${:.2} using Credit Card - Name: {}, Card Number: {}",
            amount, self.name, self.card_number
        );
    }
}

struct PayPalPayment {
    email: String,
}

impl PayPalPayment {
    fn new(email: &str) -> Self {
        PayPalPayment {
            email: email.to_string(),
        }
    }
}

impl PaymentStrategy for PayPalPayment {
    fn pay(&self, amount: f64) {
        println!("Paying ${:.2} using PayPal - Email: {}", amount, self.email);
    }
}

//Define the Decorator Pattern for Discounts
trait Order {
    fn get_total(&self) -> f64;
    fn get_description(&self) -> String;
}

struct BasicOrder {
    total: f64,
    description: String,
}

impl BasicOrder {
    fn new(total: f64, description: &str) -> Self {
        BasicOrder {
            total,
            description: description.to_string(),
        }
    }
}

impl Order for BasicOrder {
    fn get_total(&self) -> f64 {
        self.total
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }
}

struct DiscountDecorator {
    order: Box<dyn Order>,
    discount: f64,
    description: String,
}

impl DiscountDecorator {
    fn new(order: Box<dyn Order>, discount: f64, description: &str) -> Self {
        DiscountDecorator {
            order,
            discount,
            description: description.to_string(),
        }
    }
}

impl Order for DiscountDecorator {
    fn get_total(&self) -> f64 {
        self.order.get_total() - self.discount
    }

    fn get_description(&self) -> String {
        format!("{}, {}", self.order.get_description(), self.description)
    }
}

trait Observer {
    fn update(&self, order_status: &str);
}

trait Subject {
    fn register_observer(&mut self, observer: Box<dyn Observer>);
    fn remove_observer(&mut self, observer: &Box<dyn Observer>);
    fn notify_observers(&self, order_status: &str);
}

struct Customer {
    name: String,
}

impl Customer {
    fn new(name: &str) -> Self {
        Customer {
            name: name.to_string(),
        }
    }
}

impl Observer for Customer {
    fn update(&self, order_status: &str) {
        println!(
            "Customer {}: Your order status is '{}'",
            self.name, order_status
        );
    }
}

struct OrderSystem {
    observers: Vec<Box<dyn Observer>>,
}

impl OrderSystem {
    fn new() -> Self {
        OrderSystem {
            observers: Vec::new(),
        }
    }
}

impl Subject for OrderSystem {
    fn register_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer: &Box<dyn Observer>) {
        self.observers
            .retain(|obs| !std::ptr::eq(obs.as_ref(), observer.as_ref()));
    }

    fn notify_observers(&self, order_status: &str) {
        for observer in &self.observers {
            observer.update(order_status);
        }
    }
}
