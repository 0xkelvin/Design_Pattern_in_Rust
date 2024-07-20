use std::collections::HashMap;
use std::sync::Once;
use std::sync::{Arc, Mutex};

struct ConfigManager {
    settings: Mutex<HashMap<String, String>>,
}

impl ConfigManager {
    fn new() -> Arc<ConfigManager> {
        static mut SINGLETON: Option<Arc<ConfigManager>> = None;
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                let mut settings = HashMap::new();
                settings.insert("db_host".to_string(), "localhost".to_string());
                settings.insert("db_port".to_string(), "5432".to_string());
                settings.insert("api_key".to_string(), "123456".to_string());

                let config = ConfigManager {
                    settings: Mutex::new(settings),
                };
                SINGLETON = Some(Arc::new(config));
            });

            SINGLETON.clone().unwrap()
        }
    }

    fn get_setting(&self, key: &str) -> Option<String> {
        let settings = self.settings.lock().unwrap();
        settings.get(key).cloned()
    }

    fn set_setting(&self, key: &str, value: &str) {
        let mut settings = self.settings.lock().unwrap();
        settings.insert(key.to_string(), value.to_string());
    }
}

fn main() {
    let config_manager = ConfigManager::new();
    println!(
        "DB Host: {}",
        config_manager.get_setting("db_host").unwrap()
    );

    config_manager.set_setting("db_host", "127.0.0.1");
    println!(
        "Updated DB Host: {}",
        config_manager.get_setting("db_host").unwrap()
    );

    let another = ConfigManager::new();
    println!("another {}", another.get_setting("db_host").unwrap());
}
