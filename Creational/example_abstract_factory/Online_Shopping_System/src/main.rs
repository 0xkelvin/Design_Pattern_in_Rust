use std::collections::HashMap;

// AbstractProduct: PaymentProcessor
trait PaymentProcessor {
    fn process_payment(&self, amount: f64) -> String;
    fn refund_payment(&self, transaction_id: &str) -> String;
}

// ConcreteProduct: PayPalProcessor
struct PayPalProcessor;

impl PaymentProcessor for PayPalProcessor {
    fn process_payment(&self, amount: f64) -> String {
        println!("Processing PayPal payment of ${}", amount);
        "paypal_transaction_id".to_string()
    }

    fn refund_payment(&self, transaction_id: &str) -> String {
        println!("Refunding PayPal with transaction ID: {transaction_id}");
        "paypal_refund_id".to_string()
    }
}

// ConcreteProduct: StripeProcessor
struct StripeProcessor;

impl PaymentProcessor for StripeProcessor {
    fn process_payment(&self, amount: f64) -> String {
        println!("Processing Stripe payment of ${}", amount);
        "stripe_transaction_id".to_string()
    }

    fn refund_payment(&self, transaction_id: &str) -> String {
        println!("Refunding Stripe payment with transaction ID: {transaction_id}");
        "stripe_refund_id".to_string()
    }
}

// ConcreteProduct: SquareProcessor
struct SquareProcessor;

impl PaymentProcessor for SquareProcessor {
    fn process_payment(&self, amount: f64) -> String {
        println!("Processing Sqaure payment of ${amount}");
        "square_transaction_id".to_string()
    }

    fn refund_payment(&self, transaction_id: &str) -> String {
        println!("Refunding Sqaure payment with transaction ID: {transaction_id}");
        "square_refund_id".to_string()
    }
}

// AbstractProduct: Receipt
trait Receipt {
    fn generate(&self, transaction_id: &str) -> String;
}

// ConcreteProduct: PayPalReceipt
struct PayPalReceipt;

impl Receipt for PayPalReceipt {
    fn generate(&self, transaction_id: &str) -> String {
        format!("PayPal receipt for transaction ID: {transaction_id}")
    }
}

// ConcreteProduct: StripeReceipt
struct StripeReceipt;

impl Receipt for StripeReceipt {
    fn generate(&self, transaction_id: &str) -> String {
        format!("Stripe receipt for transaction ID: {transaction_id}")
    }
}

// ConcreteProduct: SquareReceipt
struct SquareReceipt;

impl Receipt for SquareReceipt {
    fn generate(&self, transaction_id: &str) -> String {
        format!("Square receipt for transaction ID: {transaction_id}")
    }
}

// AbstractFactory
trait PaymentFactory {
    fn create_processor(&self) -> Box<dyn PaymentProcessor>;
    fn create_receipt(&self) -> Box<dyn Receipt>;
}

// ConcreteFactory: PayPalFactory
struct PayPalFactory;

impl PaymentFactory for PayPalFactory {
    fn create_processor(&self) -> Box<dyn PaymentProcessor> {
        Box::new(PayPalProcessor)
    }

    fn create_receipt(&self) -> Box<dyn Receipt> {
        Box::new(PayPalReceipt)
    }
}

// ConcreteFactory: StripeFactory
struct StripeFactory;

impl PaymentFactory for StripeFactory {
    fn create_processor(&self) -> Box<dyn PaymentProcessor> {
        Box::new(StripeProcessor)
    }

    fn create_receipt(&self) -> Box<dyn Receipt> {
        Box::new(StripeReceipt)
    }
}

// ConcreteFactory: SquareFactory
struct SquareFactory;

impl PaymentFactory for SquareFactory {
    fn create_processor(&self) -> Box<dyn PaymentProcessor> {
        Box::new(SquareProcessor)
    }
    fn create_receipt(&self) -> Box<dyn Receipt> {
        Box::new(SquareReceipt)
    }
}

// Client code
fn main() {
    let payment_method = "PayPal";

    let factory: Box<dyn PaymentFactory> = match payment_method {
        "PayPal" => Box::new(PayPalFactory),
        "Stripe" => Box::new(StripeFactory),
        "Square" => Box::new(SquareFactory),
        _ => panic!("Unsupported payment method"),
    };

    let processor = factory.create_processor();
    let receipt = factory.create_receipt();

    let transaction_id = processor.process_payment(150.0);
    let receipt_text = receipt.generate(&transaction_id);

    println!("{receipt_text}");
    let refund_id = processor.refund_payment(&transaction_id);
    println!("Refund ID: {refund_id}");
}
