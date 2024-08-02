// Define the Strategy Trait
trait PaymentStrategy {
    fn pay(&self, amount: f64);
}

// Implement Concrete Strategies
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

struct BitcoinPayment {
    wallet_address: String,
}

impl BitcoinPayment {
    fn new(wallet_address: &str) -> Self {
        BitcoinPayment {
            wallet_address: wallet_address.to_string(),
        }
    }
}

impl PaymentStrategy for BitcoinPayment {
    fn pay(&self, amount: f64) {
        println!(
            "Paying ${:.2} using Bitcoin - Wallet Address: {}",
            amount, self.wallet_address
        );
    }
}

//implement the Context
struct PaymentProcessor {
    strategy: Box<dyn PaymentStrategy>,
}

impl PaymentProcessor {
    fn new(strategy: Box<dyn PaymentStrategy>) -> Self {
        PaymentProcessor { strategy }
    }

    fn set_strategy(&mut self, strategy: Box<dyn PaymentStrategy>) {
        self.strategy = strategy;
    }

    fn process_payment(&self, amount: f64) {
        self.strategy.pay(amount);
    }
}

// Use the Strategy Pattern in the Client Code
fn main() {
    let credit_card_payment = Box::new(CreditCardPayment::new("John Doe", "123-456-789-0123"));
    let paypal_payment = Box::new(PayPalPayment::new("john.doe@example.com"));
    let bitcoin_payment = Box::new(BitcoinPayment::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"));

    let mut payment_processor = PaymentProcessor::new(credit_card_payment);
    payment_processor.process_payment(250.00);

    payment_processor.set_strategy(paypal_payment);
    payment_processor.process_payment(150.00);

    payment_processor.set_strategy(bitcoin_payment);
    payment_processor.process_payment(100.00);
}
