// AbstractProduct: Button
trait Button {
    fn render(&self);
}

// ConcreteProduct: WindowsButton
struct WindowsButton;

impl Button for WindowsButton {
    fn render(&self) {
        println!("Rendering Windows button");
    }
}

// ConcreteProduct: MacOSButton
struct MacOSButton;

impl Button for MacOSButton {
    fn render(&self) {
        println!("Rendering MacOS button");
    }
}

// AbstractProduct: Checkbox
trait Checkbox {
    fn render(&self);
}

// ConcreteProduct: WindowsCheckbox
struct WindowsCheckbox;

impl Checkbox for WindowsCheckbox {
    fn render(&self) {
        println!("Rendering Windows checkbox")
    }
}

// ConcreteProduct: MacOSCheckbox
struct MacOSCheckbox;

impl Checkbox for MacOSCheckbox {
    fn render(&self) {
        println!("Rendering MacOS checkbox")
    }
}

// AbstractFactory
trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

// ConcreteFactory: WindowsFactory
struct WindowsFactory;

impl GUIFactory for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WindowsCheckbox)
    }
}

// ConcreteFactory: MacOSFactory
struct MacOSFactory;

impl GUIFactory for MacOSFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacOSButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacOSCheckbox)
    }
}

// Client code
fn main() {
    let os = "Window"; // this can be dynamic basedon runtime conditions
    let factory: Box<dyn GUIFactory> = match os {
        "Window" => Box::new(WindowsFactory),
        "MacOS" => Box::new(MacOSFactory),
        _ => panic!("Unsupported OS"),
    };

    let button = factory.create_button();
    let checkbox = factory.create_checkbox();
    button.render();
    checkbox.render();
}
