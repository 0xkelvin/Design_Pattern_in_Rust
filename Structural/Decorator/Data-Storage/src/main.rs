// Define the Component Trait
// declares common interface for wrappers and wrapped objects
trait DataSource {
    fn write_data(&mut self, data: &str);
    fn read_data(&self) -> String;
}

// Implement Concrete Components
// class of object being wrapped, it defines basic behavior which can be
// altered by decorator
struct FileDataSource {
    filename: String,
}

impl FileDataSource {
    fn new(filename: &str) -> Self {
        FileDataSource {
            filename: filename.to_string(),
        }
    }
}

impl DataSource for FileDataSource {
    fn write_data(&mut self, data: &str) {
        println!("Writing to file: {}", self.filename);
    }

    fn read_data(&self) -> String {
        println!("Reading from  file: {}", self.filename);
        "file data".to_string()
    }
}

// Implement the Decorator Structs
struct DataSourceDecorator {
    wrappe: Box<dyn DataSource>,
}

impl DataSourceDecorator {
    fn new(source: Box<dyn DataSource>) -> Self {
        DataSourceDecorator { wrappe: source }
    }
}

impl DataSource for DataSourceDecorator {
    fn write_data(&mut self, data: &str) {
        self.wrappe.write_data(data);
    }

    fn read_data(&self) -> String {
        self.wrappe.read_data()
    }
}

struct CompressionDecorator {
    wrappe: Box<dyn DataSource>,
}

impl CompressionDecorator {
    fn new(source: Box<dyn DataSource>) -> Self {
        CompressionDecorator { wrappe: source }
    }

    fn compress(&self, data: &str) -> String {
        format!("compressed({})", data)
    }

    fn decompress(&self, data: &str) -> String {
        data.replace("compressed(", "").replace(")", "")
    }
}

impl DataSource for CompressionDecorator {
    fn write_data(&mut self, data: &str) {
        let compressed_data = self.compress(data);
        self.wrappe.write_data(&compressed_data);
    }

    fn read_data(&self) -> String {
        let data = self.wrappe.read_data();
        self.decompress(&data)
    }
}

struct EncryptionDecorator {
    wrappee: Box<dyn DataSource>,
}

impl EncryptionDecorator {
    fn new(source: Box<dyn DataSource>) -> Self {
        EncryptionDecorator { wrappee: source }
    }

    fn encrypt(&self, data: &str) -> String {
        format!("encryped({})", data)
    }

    fn decrypt(&self, data: &str) -> String {
        data.replace("encrypted(", "").replace(")", "")
    }
}

impl DataSource for EncryptionDecorator {
    fn write_data(&mut self, data: &str) {
        let encrypted_data = self.encrypt(data);
        self.wrappee.write_data(&encrypted_data);
    }

    fn read_data(&self) -> String {
        let data = self.wrappee.read_data();
        self.decrypt(&data)
    }
}

struct LoggingDecorator {
    wrappee: Box<dyn DataSource>,
}

impl LoggingDecorator {
    fn new(source: Box<dyn DataSource>) -> Self {
        LoggingDecorator { wrappee: source }
    }

    fn log(&self, message: &str) {
        println!("Log: {}", message);
    }
}

impl DataSource for LoggingDecorator {
    fn write_data(&mut self, data: &str) {
        self.log(&format!("Writing data: {}", data));
        self.wrappee.write_data(data);
    }

    fn read_data(&self) -> String {
        let data = self.wrappee.read_data();
        self.log(&format!("Reading data: {}", data));
        data
    }
}

fn main() {
    let file_data_source: Box<dyn DataSource> = Box::new(FileDataSource::new("data.txt"));

    let mut compressed_data_source = CompressionDecorator::new(file_data_source);
    let mut encrypted_data_source = EncryptionDecorator::new(Box::new(compressed_data_source));
    let mut logged_data_source = LoggingDecorator::new(Box::new(encrypted_data_source));

    logged_data_source.write_data("Hello, world!");

    let result = logged_data_source.read_data();
    println!("Final data: {}", result);
}
