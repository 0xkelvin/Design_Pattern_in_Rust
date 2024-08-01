// Define the Subject and Observer Traits
use std::cell::RefCell;
use std::rc::Rc;

trait Observer {
    fn update(&self, traffic_condition: &str);
}

trait Subject {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn notify_observers(&self, traffic_condition: &str);
}

// Implement the TrafficeData Struct as the Subject
struct TrafficData {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    condition: String,
}

impl TrafficData {
    fn new() -> Self {
        TrafficData {
            observers: Vec::new(),
            condition: String::new(),
        }
    }

    fn set_condition(&mut self, condition: &str) {
        self.condition = condition.to_string();
        self.notify_observers(condition);
    }
}

impl Subject for TrafficData {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.retain(|obs| !Rc::ptr_eq(obs, &observer));
    }

    fn notify_observers(&self, traffic_condition: &str) {
        for observer in &self.observers {
            observer.borrow().update(traffic_condition);
        }
    }
}

// Implement Concrete Observers
struct TrafficLightController {
    id: String,
}

impl TrafficLightController {
    fn new(id: &str) -> Self {
        TrafficLightController { id: id.to_string() }
    }

    fn adjust_signals(&self, traffic_condition: &str) {
        println!(
            "TrafficLightController {}: Adjusting signals for traffic condition: {}",
            self.id, traffic_condition
        );
    }
}

impl Observer for TrafficLightController {
    fn update(&self, traffic_condition: &str) {
        self.adjust_signals(traffic_condition);
    }
}

struct TrafficReportGenerator {
    report: String,
}

impl TrafficReportGenerator {
    fn new() -> Self {
        TrafficReportGenerator {
            report: String::new(),
        }
    }

    fn generate_report(&self, traffic_condition: &str) {
        println!(
            "TrafficReportGenerator: Generating report for traffic condition: {}",
            traffic_condition
        );
    }
}

impl Observer for TrafficReportGenerator {
    fn update(&self, traffic_condition: &str) {
        self.generate_report(traffic_condition);
    }
}

struct EmergencyResponseUnit {
    unit_id: String,
}

impl EmergencyResponseUnit {
    fn new(unit_id: &str) -> Self {
        EmergencyResponseUnit {
            unit_id: unit_id.to_string(),
        }
    }

    fn prepare_response(&self, traffic_condition: &str) {
        println!(
            "EmergencyResponseUnit {}: Preparing response for traffic condition: {}",
            self.unit_id, traffic_condition
        );
    }
}

impl Observer for EmergencyResponseUnit {
    fn update(&self, traffic_condition: &str) {
        self.prepare_response(traffic_condition);
    }
}

// Use the Observer Pattern in the Client Code
fn main() {
    let traffic_data = Rc::new(RefCell::new(TrafficData::new()));

    let traffic_light_controller =
        Rc::new(RefCell::new(TrafficLightController::new("Controller A")));
    let traffice_report_generator = Rc::new(RefCell::new(TrafficReportGenerator::new()));
    let emergency_response_unit = Rc::new(RefCell::new(EmergencyResponseUnit::new("Unit 1")));

    traffic_data
        .borrow_mut()
        .register_observer(traffic_light_controller.clone());
    traffic_data
        .borrow_mut()
        .register_observer(traffice_report_generator.clone());
    traffic_data
        .borrow_mut()
        .register_observer(emergency_response_unit.clone());

    traffic_data.borrow_mut().set_condition("Heavy Traffice");
    traffic_data
        .borrow_mut()
        .set_condition("Accident on Highway");
}
