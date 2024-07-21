use std::collections::HashMap;
use std::sync::Once;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

struct CacheEntry {
    value: String,
    expiration: Option<SystemTime>,
}

struct Cache {
    store: Mutex<HashMap<String, CacheEntry>>,
}

impl Cache {
    fn new() -> Arc<Cache> {
        static mut SINGLETON: Option<Arc<Cache>> = None;
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                let cache = Cache {
                    store: Mutex::new(HashMap::new()),
                };
                SINGLETON = Some(Arc::new(cache));
            });

            SINGLETON.clone().unwrap()
        }
    }

    fn set(&self, key: String, value: String, ttl: Option<Duration>) {
        let expiration = ttl.map(|d| SystemTime::now() + d);
        let entry = CacheEntry { value, expiration };
        let mut store = self.store.lock().unwrap();
        store.insert(key, entry);
    }

    fn get(&self, key: &str) -> Option<String> {
        let mut store = self.store.lock().unwrap();
        if let Some(entry) = store.get(key) {
            if let Some(expiration) = entry.expiration {
                if SystemTime::now() > expiration {
                    store.remove(key);
                    return None;
                }
            }
            return Some(entry.value.clone());
        }
        None
    }
}

fn main() {
    let cache = Cache::new();

    // Set a cache entry with a TTL of 5 seconds
    cache.set(
        "key1".to_string(),
        "value1".to_string(),
        Some(Duration::new(5, 0)),
    );
    // Get the cache entry before expiration
    if let Some(value) = cache.get("key1") {
        println!("Cache hit: key1 -> {}", value);
    } else {
        println!("Cache miss: key1");
    }

    // Simulate waiting for the TTL to expire
    std::thread::sleep(Duration::new(6, 0));

    // Get the cache entry after expiration
    if let Some(value) = cache.get("key1") {
        println!("Cache hit: key1 -> {}", value);
    } else {
        println!("Cache miss: key1");
    }

    // Use the singleton instance in another part of the application
    let another_reference = Cache::new();
    another_reference.set("key2".to_string(), "value2".to_string(), None);
    println!("Set cache entry: key2 -> value2");
}
