// The Singleton pattern ensures that a class has only on instance and provides a globle
// point of access to that instance
use std::sync::Once;
use std::sync::{Arc, Mutex};

struct ConfigManager {
    settings: Mutex<String>,
}

impl ConfigManager {
    fn new() -> Arc<ConfigManager> {
        static mut SINGLETON: Option<Arc<ConfigManager>> = None;
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                let config = ConfigManager {
                    settings: Mutex::new("Default settings".to_string()),
                };
                SINGLETON = Some(Arc::new(config));
            });

            SINGLETON.clone().unwrap()
        }
    }

    fn get_settings(&self) -> String {
        let settings = self.settings.lock().unwrap();
        settings.clone()
    }

    fn set_settings(&self, settings: String) {
        let mut current_settings = self.settings.lock().unwrap();
        *current_settings = settings;
    }
}

fn main() {
    let config_manager = ConfigManager::new();
    println!("Initial settings: {}", config_manager.get_settings());

    config_manager.set_settings("New settings".to_string());

    let another_reference = ConfigManager::new();
    println!("Updated settings: {}", another_reference.get_settings());
}
