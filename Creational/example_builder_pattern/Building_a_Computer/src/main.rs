// Define the Components and Product
#[derive(Debug)]
struct Computer {
    cpu: String,
    ram: String,
    storage: String,
    gpu: Option<String>,
}

// Define the Builder Trait
trait ComputerBuilder {
    fn set_cpu(&mut self, cpu: &str) -> &mut Self;
    fn set_ram(&mut self, ram: &str) -> &mut Self;
    fn set_storage(&mut self, storage: &str) -> &mut Self;
    fn set_gpu(&mut self, gpu: &str) -> &mut Self;
    fn build(&self) -> Computer;
}

// Implement the Concrete Builder
struct CustomComputerBuilder {
    cpu: String,
    ram: String,
    storage: String,
    gpu: Option<String>,
}

impl CustomComputerBuilder {
    fn new() -> Self {
        CustomComputerBuilder {
            cpu: String::new(),
            ram: String::new(),
            storage: String::new(),
            gpu: None,
        }
    }
}

impl ComputerBuilder for CustomComputerBuilder {
    fn set_cpu(&mut self, cpu: &str) -> &mut Self {
        self.cpu = cpu.to_string();
        self
    }

    fn set_ram(&mut self, ram: &str) -> &mut Self {
        self.ram = ram.to_string();
        self
    }

    fn set_storage(&mut self, storage: &str) -> &mut Self {
        self.storage = storage.to_string();
        self
    }

    fn set_gpu(&mut self, gpu: &str) -> &mut Self {
        self.gpu = Some(gpu.to_string());
        self
    }

    fn build(&self) -> Computer {
        Computer {
            cpu: self.cpu.clone(),
            ram: self.ram.clone(),
            storage: self.storage.clone(),
            gpu: self.gpu.clone(),
        }
    }
}

// Use the Builder in the Client Code

fn main() {
    let mut builder = CustomComputerBuilder::new();

    let gaming_pc = builder
        .set_cpu("Intel Core i9")
        .set_ram("32GB")
        .set_storage("1TB SSD")
        .set_gpu("NVIDIA RTX 3080")
        .build();

    let office_pc = builder
        .set_cpu("Intel Core i5")
        .set_ram("16GB")
        .set_storage("512GB SSD")
        .build();

    println!("Gaming PC: {:?}", gaming_pc);
    println!("Office PC: {:?}", office_pc);
}
