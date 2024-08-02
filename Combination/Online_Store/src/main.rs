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

// Use the Combined Patterns in the Client Code
fn main() {
    // Strategy pattern: payment methods
    let credit_card_payment = Box::new(CreditCardPayment::new("John Doe", "1234-5678-9012-3456"));
    let paypal_payment = Box::new(PayPalPayment::new("john.doe@example.com"));

    // Decorator pattern: order with discounts
    let basic_order: Box<dyn Order> = Box::new(BasicOrder::new(100.0, "Basic Order"));
    let discounted_order = DiscountDecorator::new(basic_order, 10.0, "Holiday Discount");

    // Observer pattern: notifications
    let mut order_system = OrderSystem::new();
    let customer1 = Box::new(Customer::new("Alice"));
    let customer2 = Box::new(Customer::new("Bob"));

    order_system.register_observer(customer1);
    order_system.register_observer(customer2);

    // Process order with credit card payment
    println!("Processing order with credit card:");
    credit_card_payment.pay(discounted_order.get_total());
    order_system.notify_observers("Order placed with credit card");

    // Process order with PayPal payment
    println!("\nProcessing order with PayPal:");
    paypal_payment.pay(discounted_order.get_total());
    order_system.notify_observers("Order placed with PayPal");

    // Show final order details
    println!("\nFinal Order Details:");
    println!("Description: {}", discounted_order.get_description());
    println!("Total: ${:.2}", discounted_order.get_total());
}
