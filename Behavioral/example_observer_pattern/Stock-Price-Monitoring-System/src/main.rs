// Define the Subject and Observer Traits
use std::cell::RefCell;
use std::rc::Rc;

trait Observer {
    fn update(&mut self, price: f64);
}

trait Subject {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn notify_observers(&self);
}

// Implement the StockData Struct as the Subject
struct StockData {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    price: f64,
}

impl StockData {
    fn new() -> Self {
        StockData {
            observers: Vec::new(),
            price: 0.0,
        }
    }

    fn set_price(&mut self, price: f64) {
        self.price = price;
        self.notify_observers();
    }
}

impl Subject for StockData {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.retain(|obs| !Rc::ptr_eq(obs, &observer));
    }

    fn notify_observers(&self) {
        for observer in &self.observers {
            observer.borrow_mut().update(self.price);
        }
    }
}

// Implement Concrete Observers
struct CurrentPricesDisplay {
    price: f64,
}

impl CurrentPricesDisplay {
    fn new() -> Self {
        CurrentPricesDisplay { price: 0.0 }
    }

    fn display(&self) {
        println!("Current price: ${:.2}", self.price);
    }
}

impl Observer for CurrentPricesDisplay {
    fn update(&mut self, price: f64) {
        self.price = price;
        self.display();
    }
}

struct PercentageChangeDisplay {
    last_price: Option<f64>,
    change: f64,
}

impl PercentageChangeDisplay {
    fn new() -> Self {
        PercentageChangeDisplay {
            last_price: None,
            change: 0.0,
        }
    }

    fn display(&self) {
        println!("Percentage change: {:.2}%", self.change);
    }
}

impl Observer for PercentageChangeDisplay {
    fn update(&mut self, price: f64) {
        if let Some(last_price) = self.last_price {
            self.change = ((price - last_price) / last_price) * 100.0;
        }
        self.last_price = Some(price);
        self.display();
    }
}

// Use the Observer Pattern in the Client Code
fn main() {
    let stock_data = Rc::new(RefCell::new(StockData::new()));

    let current_price_display = Rc::new(RefCell::new(CurrentPricesDisplay::new()));
    let percentage_change_display = Rc::new(RefCell::new(PercentageChangeDisplay::new()));

    stock_data
        .borrow_mut()
        .register_observer(current_price_display.clone());
    stock_data
        .borrow_mut()
        .register_observer(percentage_change_display.clone());

    stock_data.borrow_mut().set_price(100.0);
    stock_data.borrow_mut().set_price(105.0);
    stock_data.borrow_mut().set_price(110.0);

    stock_data
        .borrow_mut()
        .remove_observer(current_price_display.clone());
    stock_data.borrow_mut().set_price(120.0);
}
