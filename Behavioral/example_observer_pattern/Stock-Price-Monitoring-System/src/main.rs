// Define the Subject and Observer Traits
use std::cell::{self, RefCell}:RefCell;
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
//
