// Abstract Products
trait PaymentSession {
    fn create_session(&self);
}

trait PaymentProcessor {
    fn process_payment(&self, amount: f64);
}

trait ReceiptHandler {
    fn handle_receipt(&self, receipt: &str);
}

// Concrete Products for Credit Card
struct CreditCardPaymentSession;

impl PaymentSession for CreditCardPaymentSession {
    fn create_session(&self) {
        println!("Creating credit card payment session");
    }
}

struct CreditCardPaymentProcessor;

impl PaymentProcessor for CreditCardPaymentProcessor {
    fn process_payment(&self, amount: f64) {
        println!("Processing credit card payment of ${amount}");
    }
}

struct CreditCardReceiptHandler;

impl ReceiptHandler for CreditCardReceiptHandler {
    fn handle_receipt(&self, receipt: &str) {
        println!("Handling credit card receipt: {receipt}");
    }
}

// Concrete Products for PayPal
struct PayPalPaymentSession;

impl PaymentSession for PayPalPaymentSession {
    fn create_session(&self) {
        println!("Creating PayPal payment session");
    }
}

struct PayPalPaymentProcessor;

impl PaymentProcessor for PayPalPaymentProcessor {
    fn process_payment(&self, amount: f64) {
        println!("Processing PayPal payment of ${}", amount);
    }
}

struct PayPalReceiptHandler;

impl ReceiptHandler for PayPalReceiptHandler {
    fn handle_receipt(&self, receipt: &str) {
        println!("Handling PayPal receipt: {}", receipt);
    }
}

// Concrete Products for Bank Transfer

struct BankTransferPaymentSession;

impl PaymentSession for BankTransferPaymentSession {
    fn create_session(&self) {
        println!("Creating bank transfer payment session");
    }
}

struct BankTransferPaymentProcessor;

impl PaymentProcessor for BankTransferPaymentProcessor {
    fn process_payment(&self, amount: f64) {
        println!("Processing bank transfer payment of ${}", amount);
    }
}

struct BankTransferReceiptHandler;

impl ReceiptHandler for BankTransferReceiptHandler {
    fn handle_receipt(&self, receipt: &str) {
        println!("Handling bank transfer receipt: {}", receipt);
    }
}

// Abstract Factory
trait PaymentFactory {
    fn create_payment_session(&self) -> Box<dyn PaymentSession>;
    fn create_payment_processor(&self) -> Box<dyn PaymentProcessor>;
    fn create_receipt_handler(&self) -> Box<dyn ReceiptHandler>;
}

// Concrete Factory
struct CreditCardFactory;

impl PaymentFactory for CreditCardFactory {
    fn create_payment_session(&self) -> Box<dyn PaymentSession> {
        Box::new(CreditCardPaymentSession)
    }

    fn create_payment_processor(&self) -> Box<dyn PaymentProcessor> {
        Box::new(CreditCardPaymentProcessor)
    }

    fn create_receipt_handler(&self) -> Box<dyn ReceiptHandler> {
        Box::new(CreditCardReceiptHandler)
    }
}

struct PayPalFactory;

impl PaymentFactory for PayPalFactory {
    fn create_payment_session(&self) -> Box<dyn PaymentSession> {
        Box::new(PayPalPaymentSession)
    }

    fn create_payment_processor(&self) -> Box<dyn PaymentProcessor> {
        Box::new(PayPalPaymentProcessor)
    }

    fn create_receipt_handler(&self) -> Box<dyn ReceiptHandler> {
        Box::new(PayPalReceiptHandler)
    }
}

struct BankTransferFactory;

impl PaymentFactory for BankTransferFactory {
    fn create_payment_session(&self) -> Box<dyn PaymentSession> {
        Box::new(BankTransferPaymentSession)
    }

    fn create_payment_processor(&self) -> Box<dyn PaymentProcessor> {
        Box::new(BankTransferPaymentProcessor)
    }

    fn create_receipt_handler(&self) -> Box<dyn ReceiptHandler> {
        Box::new(BankTransferReceiptHandler)
    }
}

//Client Code
fn main() {
    let payment_type = "PayPal";

    let factory: Box<dyn PaymentFactory> = match payment_type {
        "CreditCard" => Box::new(CreditCardFactory),
        "PayPal" => Box::new(PayPalFactory),
        "BankTransfer" => Box::new(BankTransferFactory),
        _ => panic!("Unsupported payment type"),
    };

    let payment_session = factory.create_payment_session();
    let payment_processor = factory.create_payment_processor();
    let receipt_handler = factory.create_receipt_handler();

    payment_session.create_session();
    payment_processor.process_payment(100.0);
    receipt_handler.handle_receipt("Payment successful")
}
