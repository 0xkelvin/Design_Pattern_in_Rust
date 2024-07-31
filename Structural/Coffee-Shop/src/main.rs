// Define the Component Trait
trait Coffee {
    fn cost(&self) -> f64;
    fn description(&self) -> String;
}

// Implement Concrete Components
struct Espresso;

impl Coffee for Espresso {
    fn cost(&self) -> f64 {
        1.99
    }

    fn description(&self) -> String {
        "Espresso".to_string()
    }
}

struct HouseBlend;

impl Coffee for HouseBlend {
    fn cost(&self) -> f64 {
        0.89
    }

    fn description(&self) -> String {
        "House Blend Coffee".to_string()
    }
}

// Implement the Decorator Struct
struct CondimentDecorator {
    coffee: Box<dyn Coffee>,
    condiment: String,
    condiment_cost: f64,
}

impl Coffee for CondimentDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost() + self.condiment_cost
    }

    fn description(&self) -> String {
        format!("{}, {}", self.coffee.description(), self.condiment)
    }
}

impl CondimentDecorator {
    fn new(coffee: Box<dyn Coffee>, condiment: &str, condiment_cost: f64) -> Self {
        CondimentDecorator {
            coffee,
            condiment: condiment.to_string(),
            condiment_cost,
        }
    }
}

fn main() {
    let mut coffee: Box<dyn Coffee> = Box::new(Espresso);
    println!(
        "Description: {}, Cost: ${:.2}",
        coffee.description(),
        coffee.cost()
    );

    coffee = Box::new(CondimentDecorator::new(coffee, "Milk", 0.10));
    println!(
        "Description: {}, Cost: ${:.2}",
        coffee.description(),
        coffee.cost()
    );

    coffee = Box::new(CondimentDecorator::new(coffee, "Soy", 0.15));
    println!(
        "Description: {}, Cost: ${:.2}",
        coffee.description(),
        coffee.cost()
    );

    coffee = Box::new(CondimentDecorator::new(coffee, "Mocha", 0.20));
    println!(
        "Description: {}, Cost: ${:.2}",
        coffee.description(),
        coffee.cost()
    );

    coffee = Box::new(CondimentDecorator::new(coffee, "Whipped Cream", 0.30));
    println!(
        "Description: {}, Cost: ${:.2}",
        coffee.description(),
        coffee.cost()
    );

    let mut another_coffee: Box<dyn Coffee> = Box::new(HouseBlend);
    println!(
        "Description: {}, Cost: ${:.2}",
        another_coffee.description(),
        another_coffee.cost()
    );

    another_coffee = Box::new(CondimentDecorator::new(another_coffee, "Mocha", 0.20));
    println!(
        "Description: {}, Cost: ${:.2}",
        another_coffee.description(),
        another_coffee.cost()
    );

    another_coffee = Box::new(CondimentDecorator::new(
        another_coffee,
        "Whipped Cream",
        0.30,
    ));
    println!(
        "Description: {}, Cost: ${:.2}",
        another_coffee.description(),
        another_coffee.cost()
    );
}
