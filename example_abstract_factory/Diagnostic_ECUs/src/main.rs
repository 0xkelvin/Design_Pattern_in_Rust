use std::any::Any;
use std::collections::HashMap;

// AbstractProduct: ECU
trait ECU: Any {
    fn connect(&self);
    fn configure(&self, config: &dyn ECUConfiguration);
    fn as_any(&self) -> &dyn Any;
}

// ConcreteProduct: BMS
struct BMS;

impl ECU for BMS {
    fn connect(&self) {
        println!("Connecting to BMS via system CAN bus");
    }

    fn configure(&self, config: &dyn ECUConfiguration) {
        println!("Configuring BMS with setting: {:?}", config.get_settings());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ConcreteProduct: MotorController
struct MotorController;

impl ECU for MotorController {
    fn connect(&self) {
        println!("Connecting to Motor Controller via system CAN bus");
    }

    fn configure(&self, config: &dyn ECUConfiguration) {
        println!(
            "Configuring Motor Controller with setting: {:?}",
            config.get_settings()
        );
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ConcreteProduct: OnBoardCharger
struct OnBoardCharger;

impl ECU for OnBoardCharger {
    fn connect(&self) {
        println!("Connecting to On Board Charger via system CAN bus");
    }

    fn configure(&self, config: &dyn ECUConfiguration) {
        println!(
            "Configuring On Board Charger with settings: {:?}",
            config.get_settings()
        );
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ConcreteProduct: VehicleControlECU
struct VehicleControlECU {
    connected_ecus: Vec<Box<dyn ECU>>,
}

impl ECU for VehicleControlECU {
    fn connect(&self) {
        println!("Connecting to Vehicle Control ECU via Bluetooth/Wi-Fi/4G/Diag CAN/OTG USB");
    }

    fn configure(&self, config: &dyn ECUConfiguration) {
        println!(
            "Configuring Vehicle Control ECU with settings: {:?}",
            config.get_settings()
        );
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl VehicleControlECU {
    fn new() -> Self {
        Self {
            connected_ecus: vec![
                Box::new(BMS),
                Box::new(MotorController),
                Box::new(OnBoardCharger),
            ],
        }
    }

    fn connect_to_all_ecus(&self) {
        for ecu in &self.connected_ecus {
            ecu.connect();
        }
    }

    fn configure_all_ecus(&self, config: &dyn ECUConfiguration) {
        for ecu in &self.connected_ecus {
            ecu.configure(config);
        }
    }
}

// AbstractProduct: ECUConfiguration
trait ECUConfiguration {
    fn set_settings(&mut self, settings: HashMap<String, String>);
    fn get_settings(&self) -> HashMap<String, String>;
}

// ConcreteProduct: BMSConfiguration
struct BMSConfiguration {
    settings: HashMap<String, String>,
}

impl ECUConfiguration for BMSConfiguration {
    fn set_settings(&mut self, settings: HashMap<String, String>) {
        self.settings = settings;
    }

    fn get_settings(&self) -> HashMap<String, String> {
        self.settings.clone()
    }
}

// ConcreteProduct: MotorControllerConfiguration
struct MotorControllerConfiguration {
    settings: HashMap<String, String>,
}

impl ECUConfiguration for MotorControllerConfiguration {
    fn set_settings(&mut self, settings: HashMap<String, String>) {
        self.settings = settings;
    }

    fn get_settings(&self) -> HashMap<String, String> {
        self.settings.clone()
    }
}

// ConcreteProduct: OnBoardChargerConfiguration
struct OnBoardChargerConfiguration {
    settings: HashMap<String, String>,
}

impl ECUConfiguration for OnBoardChargerConfiguration {
    fn set_settings(&mut self, settings: HashMap<String, String>) {
        self.settings = settings;
    }

    fn get_settings(&self) -> HashMap<String, String> {
        self.settings.clone()
    }
}

// ConcreteProduct: VehicleControlECUConfiguration
struct VehicleControlECUConfiguration {
    settings: HashMap<String, String>,
}

impl ECUConfiguration for VehicleControlECUConfiguration {
    fn set_settings(&mut self, settings: HashMap<String, String>) {
        self.settings = settings;
    }

    fn get_settings(&self) -> HashMap<String, String> {
        self.settings.clone()
    }
}

// AbstractProduct: ECUDiagnostics
trait ECUDiagnostics {
    fn run_diagnostics(&self) -> HashMap<String, String>;
}

// ConcreteProduct: BMSDiagnostics
struct BMSDiagnostics;

impl ECUDiagnostics for BMSDiagnostics {
    fn run_diagnostics(&self) -> HashMap<String, String> {
        let mut diagnostics = HashMap::new();
        diagnostics.insert("voltage".to_string(), "400V".to_string());
        diagnostics.insert("current".to_string(), "100A".to_string());
        diagnostics
    }
}

// ConcreteProduct: MotorControllerDiagnostics
struct MotorControllerDiagnostics;

impl ECUDiagnostics for MotorControllerDiagnostics {
    fn run_diagnostics(&self) -> HashMap<String, String> {
        let mut diagnostics = HashMap::new();
        diagnostics.insert("rpm".to_string(), "3000".to_string());
        diagnostics.insert("torque".to_string(), "250Nm".to_string());
        diagnostics
    }
}

// ConcreteProduct: OnBoardChargerDiagnostics
struct OnBoardChargerDiagnostics;

impl ECUDiagnostics for OnBoardChargerDiagnostics {
    fn run_diagnostics(&self) -> HashMap<String, String> {
        let mut diagnostics = HashMap::new();
        diagnostics.insert("input_voltage".to_string(), "220V".to_string());
        diagnostics.insert("charging_current".to_string(), "50A".to_string());
        diagnostics
    }
}

// ConcreteProduct: VehicleControlECUDiagnostics
struct VehicleControlECUDiagnostics;

impl ECUDiagnostics for VehicleControlECUDiagnostics {
    fn run_diagnostics(&self) -> HashMap<String, String> {
        let mut diagnostics = HashMap::new();
        diagnostics.insert("gps_status".to_string(), "Connected".to_string());
        diagnostics.insert("network_status".to_string(), "4G".to_string());
        diagnostics
    }
}

// AbstractProduct: ECUDataCollector
trait ECUDataCollector {
    fn collect_data(&self) -> HashMap<String, String>;
}

// ConcreteProduct: BMSDataCollector
struct BMSDataCollector;

impl ECUDataCollector for BMSDataCollector {
    fn collect_data(&self) -> HashMap<String, String> {
        let mut data = HashMap::new();
        data.insert(
            "cell_voltages".to_string(),
            "3.7V, 3.7V, 3.6V, ...".to_string(),
        );
        data.insert("temperatures".to_string(), "30C, 30C, 31C, ...".to_string());
        data
    }
}

// ConcreteProduct: MotorControllerDataCollector
struct MotorControllerDataCollector;

impl ECUDataCollector for MotorControllerDataCollector {
    fn collect_data(&self) -> HashMap<String, String> {
        let mut data = HashMap::new();
        data.insert("motor_temp".to_string(), "85C".to_string());
        data.insert("inverter_temp".to_string(), "75C".to_string());
        data
    }
}

// ConcreteProduct: OnBoardChargerDataCollector
struct OnBoardChargerDataCollector;

impl ECUDataCollector for OnBoardChargerDataCollector {
    fn collect_data(&self) -> HashMap<String, String> {
        let mut data = HashMap::new();
        data.insert("charging_time".to_string(), "2 hours".to_string());
        data.insert("energy_delivered".to_string(), "10 kWh".to_string());
        data
    }
}

// ConcreteProduct: VehicleControlECUDataCollector
struct VehicleControlECUDataCollector;

impl ECUDataCollector for VehicleControlECUDataCollector {
    fn collect_data(&self) -> HashMap<String, String> {
        let mut data = HashMap::new();
        data.insert(
            "gps_location".to_string(),
            "Lat: 40.7128, Long: -74.0060".to_string(),
        );
        data.insert("network_type".to_string(), "4G".to_string());
        data
    }
}

// AbstractFactory
trait ECUFactory {
    fn create_ecu(&self) -> Box<dyn ECU>;
    fn create_configuration(&self) -> Box<dyn ECUConfiguration>;
    fn create_diagnostics(&self) -> Box<dyn ECUDiagnostics>;
    fn create_data_collector(&self) -> Box<dyn ECUDataCollector>;
}

// ConcreteFactory: BMSFactory
struct BMSFactory;

impl ECUFactory for BMSFactory {
    fn create_ecu(&self) -> Box<dyn ECU> {
        Box::new(BMS)
    }

    fn create_configuration(&self) -> Box<dyn ECUConfiguration> {
        Box::new(BMSConfiguration {
            settings: HashMap::new(),
        })
    }

    fn create_diagnostics(&self) -> Box<dyn ECUDiagnostics> {
        Box::new(BMSDiagnostics)
    }

    fn create_data_collector(&self) -> Box<dyn ECUDataCollector> {
        Box::new(BMSDataCollector)
    }
}

// ConcreteFactory: MotorControllerFactory
struct MotorControllerFactory;

impl ECUFactory for MotorControllerFactory {
    fn create_ecu(&self) -> Box<dyn ECU> {
        Box::new(MotorController)
    }

    fn create_configuration(&self) -> Box<dyn ECUConfiguration> {
        Box::new(MotorControllerConfiguration {
            settings: HashMap::new(),
        })
    }

    fn create_diagnostics(&self) -> Box<dyn ECUDiagnostics> {
        Box::new(MotorControllerDiagnostics)
    }

    fn create_data_collector(&self) -> Box<dyn ECUDataCollector> {
        Box::new(MotorControllerDataCollector)
    }
}

// ConcreteFactory: OnBoardChargerFactory
struct OnBoardChargerFactory;

impl ECUFactory for OnBoardChargerFactory {
    fn create_ecu(&self) -> Box<dyn ECU> {
        Box::new(OnBoardCharger)
    }

    fn create_configuration(&self) -> Box<dyn ECUConfiguration> {
        Box::new(OnBoardChargerConfiguration {
            settings: HashMap::new(),
        })
    }

    fn create_diagnostics(&self) -> Box<dyn ECUDiagnostics> {
        Box::new(OnBoardChargerDiagnostics)
    }

    fn create_data_collector(&self) -> Box<dyn ECUDataCollector> {
        Box::new(OnBoardChargerDataCollector)
    }
}

// ConcreteFactory: VehicleControlECUFactory
struct VehicleControlECUFactory;

impl ECUFactory for VehicleControlECUFactory {
    fn create_ecu(&self) -> Box<dyn ECU> {
        Box::new(VehicleControlECU::new())
    }

    fn create_configuration(&self) -> Box<dyn ECUConfiguration> {
        Box::new(VehicleControlECUConfiguration {
            settings: HashMap::new(),
        })
    }

    fn create_diagnostics(&self) -> Box<dyn ECUDiagnostics> {
        Box::new(VehicleControlECUDiagnostics)
    }

    fn create_data_collector(&self) -> Box<dyn ECUDataCollector> {
        Box::new(VehicleControlECUDataCollector)
    }
}

fn main() {
    let ecu_type = "VehicleControlECU";

    let factory: Box<dyn ECUFactory> = match ecu_type {
        "BMS" => Box::new(BMSFactory),
        "MotorController" => Box::new(MotorControllerFactory),
        "OnBoardCharger" => Box::new(OnBoardChargerFactory),
        "VehicleControlECU" => Box::new(VehicleControlECUFactory),
        _ => panic!("Unsupported ECU type"),
    };

    let vehicle_control_ecu = factory.create_ecu();
    let mut configuration = factory.create_configuration();
    let diagnostics = factory.create_diagnostics();
    let data_collector = factory.create_data_collector();

    vehicle_control_ecu.connect();

    let mut settings = HashMap::new();
    settings.insert("setting1".to_string(), "value1".to_string());
    configuration.set_settings(settings.clone());
    vehicle_control_ecu.configure(&*configuration);

    // Connect and configure all ECUs through Vehicle Control ECU
    if let Some(vce) = vehicle_control_ecu
        .as_any()
        .downcast_ref::<VehicleControlECU>()
    {
        vce.connect_to_all_ecus();
        vce.configure_all_ecus(&*configuration);
    }

    println!("Running Diagnostics: {:?}", diagnostics.run_diagnostics());
    println!("Collecting Data: {:?}", data_collector.collect_data());
}
