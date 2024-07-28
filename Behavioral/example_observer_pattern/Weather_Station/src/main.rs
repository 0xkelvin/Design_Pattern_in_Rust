//magine you are building a weather monitoring system where multiple display devices (observers)
//need to be updated whenever the weather data (subject) changes.
//Each display device might show the weather data in a different format (e.g., current conditions, statistics, forecast).

// Define the Subject and Observer Traits
use std::cell::RefCell;
use std::rc::Rc;

trait Observer {
    fn update(&mut self, temperature: f64, humidity: f64, pressure: f64);
}

trait Subject {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn notify_observers(&self);
}

// Implement the WeatherData Struct as the Subject
struct WeatherData {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    temperature: f64,
    humidity: f64,
    pressure: f64,
}

impl WeatherData {
    fn new() -> Self {
        WeatherData {
            observers: Vec::new(),
            temperature: 0.0,
            humidity: 0.0,
            pressure: 0.0,
        }
    }

    fn set_measurements(&mut self, temperature: f64, humidity: f64, pressure: f64) {
        self.temperature = temperature;
        self.humidity = humidity;
        self.pressure = pressure;
        self.notify_observers();
    }
}

impl Subject for WeatherData {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.retain(|obs| !Rc::ptr_eq(obs, &observer));
    }

    fn notify_observers(&self) {
        for observer in &self.observers {
            observer
                .borrow_mut()
                .update(self.temperature, self.humidity, self.pressure);
        }
    }
}

// Implement Concrete Observers
struct CurrentConditionsDisplay {
    temperature: f64,
    humidity: f64,
}

impl CurrentConditionsDisplay {
    fn new() -> Self {
        CurrentConditionsDisplay {
            temperature: 0.0,
            humidity: 0.0,
        }
    }

    fn display(&self) {
        println!(
            "Current conditions: {:.2}F degrees and {:.2}% humidity",
            self.temperature, self.humidity
        );
    }
}

impl Observer for CurrentConditionsDisplay {
    fn update(&mut self, temperature: f64, humidity: f64, _pressure: f64) {
        self.temperature = temperature;
        self.humidity = humidity;
        self.display();
    }
}

struct StatisticsDisplay {
    max_temp: f64,
    min_temp: f64,
    temp_sum: f64,
    num_reading: u32,
}

impl StatisticsDisplay {
    fn new() -> Self {
        StatisticsDisplay {
            max_temp: std::f64::MAX,
            min_temp: std::f64::MIN,
            temp_sum: 0.0,
            num_reading: 0,
        }
    }

    fn display(&self) {
        let avg_temp = self.temp_sum / self.num_reading as f64;
        println!(
            "Avg/Max/Min temperature = {:.2}/{:.2}/{:.2}",
            avg_temp, self.max_temp, self.min_temp
        );
    }
}

impl Observer for StatisticsDisplay {
    fn update(&mut self, temperature: f64, _humidity: f64, _pressure: f64) {
        self.temp_sum += temperature;
        self.num_reading += 1;
        if temperature > self.max_temp {
            self.max_temp = temperature;
        }

        if temperature < self.min_temp {
            self.min_temp = temperature;
        }

        self.display();
    }
}

// Use the Observer Pattern in the Client Code
fn main() {
    let weather_data = Rc::new(RefCell::new(WeatherData::new()));

    let current_display = Rc::new(RefCell::new(CurrentConditionsDisplay::new()));
    let statistics_display = Rc::new(RefCell::new(StatisticsDisplay::new()));

    weather_data
        .borrow_mut()
        .register_observer(current_display.clone());
    weather_data
        .borrow_mut()
        .register_observer(statistics_display.clone());

    weather_data.borrow_mut().set_measurements(80.0, 65.0, 30.4);
    weather_data.borrow_mut().set_measurements(82.0, 70.0, 29.2);
    weather_data.borrow_mut().set_measurements(78.0, 90.0, 29.2);

    weather_data
        .borrow_mut()
        .remove_observer(current_display.clone());
    weather_data.borrow_mut().set_measurements(85.0, 75.0, 28.2);
}
