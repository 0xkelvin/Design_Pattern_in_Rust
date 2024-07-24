use serde::Deserialize;
use std::fs;
use std::sync::{Arc, Mutex, Once};

#[derive(Debug, Deserialize, Clone)]
struct Config {
    database_url: String,
    cache_size: usize,
    max_connections: usize,
}

struct ConfigManager {
    config: Mutex<Option<Config>>,
}

impl ConfigManager {
    fn new() -> Arc<ConfigManager> {
        static mut SINGLETON: Option<Arc<ConfigManager>> = None;
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                let manager = ConfigManager {
                    config: Mutex::new(None),
                };
                SINGLETON = Some(Arc::new(manager));
            });
            SINGLETON.clone().unwrap()
        }
    }

    fn load_config(&self, file_path: &str) {
        let config_data = fs::read_to_string(file_path).expect("Unable to read config file");
        let config: Config = toml::from_str(&config_data).expect("Unable to parse config file");
        let mut config_lock = self.config.lock().unwrap();
        *config_lock = Some(config)
    }

    fn get_config(&self) -> Config {
        let config_lock = self.config.lock().unwrap();
        config_lock.clone().expect("Config not loaded")
    }
}

fn main() {
    // Load configuration
    let config_manager = ConfigManager::new();
    config_manager.load_config("config.toml");

    // Access configuration
    let config = config_manager.get_config();
    println!("Database URL: {}", config.database_url);
    println!("Cache Size: {}", config.cache_size);
    println!("Max Connections: {}", config.max_connections);

    // Use the singleton instance in another part of the application
    let another_reference = ConfigManager::new();
    let another_config = another_reference.get_config();
    println!("Accessing config from another reference:");
    println!("Database URL: {}", another_config.database_url);
    println!("Cache Size: {}", another_config.cache_size);
    println!("Max Connections: {}", another_config.max_connections);
}
