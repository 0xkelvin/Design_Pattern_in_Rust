// Define the Product trait, common operations that all payment methods should support. 
trait PaymentMethod {
    fn process_payment(&self, amount: f64);
}

// ConcreteProduct: CreditCardPayment, specific implementation for payments
struct CreditCardPayment;

impl PaymentMethod for CreditCardPayment {
    fn process_payment(&self, amount: f64) {
        println!("Processing credit card payment for amount: {}", amount);
    }
}

// ConcreteProduct: PaypalPayment, specific implementation for payments
struct PaypalPayment;

impl PaymentMethod for PaypalPayment {
    fn process_payment(&self, amount: f64) {
        println!("Processing Paypal payment for amount: {}", amount);
    }
}

// ConcreteProduct: Bitcoin, specific implementation for payments
struct BitcoinPayment; 

impl PaymentMethod for BitcoinPayment {
    fn process_payment(&self, amount: f64) {
        println!("Processing Bitcoin payment for amount: {}", amount);
    }
}


// Creator, method for creating payment methods
trait PaymentFactory {
    fn create_payment(&self) -> Box<dyn PaymentMethod>;
}

// ConcreteCreator: CreditCardPaymentFactory
struct CreditCardPaymentFactory;

impl PaymentFactory for CreditCardPaymentFactory {
    fn create_payment(&self) -> Box<dyn PaymentMethod> {
        Box::new(CreditCardPayment)
    }
}

// ConcreteCreator: PaypalPaymentFactory
struct PaypalPaymentFactory;

impl PaymentFactory for PaypalPaymentFactory {
    fn create_payment(&self) -> Box<dyn PaymentMethod> {
        Box::new(PaypalPayment)
    }
}

// ConcreteCreator: BitcoinPaymentFactory
struct BitcoinPaymentFactory;

impl PaymentFactory for BitcoinPaymentFactory {
    fn create_payment(&self) -> Box<dyn PaymentMethod> {
        Box::new(BitcoinPayment)
    }
}

fn main() {
    let credit_card_factory = CreditCardPaymentFactory;
    let paypal_factory = PaypalPaymentFactory;
    let bitcoin_factory = BitcoinPaymentFactory;

    let payments = vec![
        credit_card_factory.create_payment(),
        paypal_factory.create_payment(),
        bitcoin_factory.create_payment(),
    ];

    for payment in payments {
        payment.process_payment(100.0);
    }
}
