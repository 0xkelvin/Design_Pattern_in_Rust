// Define the Product trait
trait Coffee {
    fn brew(&self);
}

// ConcreteProduct: Espresso
struct Espresso;

impl Coffee for Espresso {
    fn brew(&self) {
        println!("Brewing Espresso");
    }
}

// ConcreteProduct: Cappuccino 
struct Cappuccino;

impl Coffee for Cappuccino {
    fn brew(&self) {
        println!("Brewing Cappuccino");
    }
}

// ConcreteProduct: Latte
struct Latte;

impl Coffee for Latte {
    fn brew(&self) {
        println!("Brewing Latte");
    }
}

// Define the Creator trait
trait CoffeeFactory {
    fn create_coffee(&self) -> Box<dyn Coffee>;
}

// ConcreteCreator: EspressoFactory
struct EspressoFactory;

impl CoffeeFactory for EspressoFactory {
    fn create_coffee(&self) -> Box<dyn Coffee> {
        Box::new(Espresso)
    }
}


// ConcreteCreator: CappuccinoFactory
struct CappuccinoFactory;

impl CoffeeFactory for CappuccinoFactory {
    fn create_coffee(&self) -> Box<dyn Coffee> {
        Box::new(Cappuccino)
    }
}

// ConcreteCreator: LatteFactory
struct LatteFactory;    
impl CoffeeFactory for LatteFactory {
    fn create_coffee(&self) -> Box<dyn Coffee> {
        Box::new(Latte)
    }
}

fn main() {
    let espresso_factory = EspressoFactory;
    let cappuccino_factory = CappuccinoFactory;
    let latte_factory = LatteFactory;

    let espresso = espresso_factory.create_coffee();
    let cappuccino = cappuccino_factory.create_coffee();
    let latte = latte_factory.create_coffee();

    espresso.brew();
    cappuccino.brew();
    latte.brew();
}
