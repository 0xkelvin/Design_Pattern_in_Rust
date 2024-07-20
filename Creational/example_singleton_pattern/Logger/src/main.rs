use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Once;
use std::sync::{Arc, Mutex};

struct Logger {
    file: Mutex<std::fs::File>,
}

impl Logger {
    fn new() -> Arc<Logger> {
        static mut SINGLETON: Option<Arc<Logger>> = None;
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                let file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open("log.txt")
                    .unwrap();
                let logger = Logger {
                    file: Mutex::new(file),
                };
                SINGLETON = Some(Arc::new(logger));
            });

            SINGLETON.clone().unwrap()
        }
    }

    fn log(&self, message: &str) {
        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", message).unwrap();
    }
}

fn main() {
    let logger = Logger::new();
    logger.log("This is the first log message");

    let another_reference = Logger::new();
    another_reference.log("This is the second log message");
}
