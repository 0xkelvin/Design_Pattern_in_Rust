// Abstract Products
trait FileReader {
    fn read(&self, path: &str) -> String;
}

trait FileWriter {
    fn write(&self, path: &str, content: &str);
}

trait DirectoryHandler {
    fn create_directory(&self, path: &str);
}

// Concrete Products for Windows
struct WindowsFileReader;

impl FileReader for WindowsFileReader {
    fn read(&self, path: &str) -> String {
        println!("Reading file from windows file system at {}", path);
        "Windows file content".to_string()
    }
}

struct WindowsFileWriter;

impl FileWriter for WindowsFileWriter {
    fn write(&self, path: &str, content: &str) {
        println!("Writing to file in Windows file system at {}", path);
        println!("Content: {}", content);
    }
}

struct WindowsDirectoryHandler;

impl DirectoryHandler for WindowsDirectoryHandler {
    fn create_directory(&self, path: &str) {
        println!("Creating directory in Window file system at {}", path);
    }
}

struct LinuxFileReader;

impl FileReader for LinuxFileReader {
    fn read(&self, path: &str) -> String {
        println!("Reading file from Linux file system at {}", path);
        "Linux file content".to_string()
    }
}

struct LinuxFileWriter;

impl FileWriter for LinuxFileWriter {
    fn write(&self, path: &str, content: &str) {
        println!("Writing to file in Linux file system at {}", path);
        println!("Content: {}", content);
    }
}

struct LinuxDirectoryHandler;

impl DirectoryHandler for LinuxDirectoryHandler {
    fn create_directory(&self, path: &str) {
        println!("Creating directory in Linux file system at {}", path);
    }
}

// Concrete Products for macOS
struct MacOSFileReader;

impl FileReader for MacOSFileReader {
    fn read(&self, path: &str) -> String {
        println!("Reading file from macOS file system at {}", path);
        "macOS file content".to_string()
    }
}

struct MacOSFileWriter;

impl FileWriter for MacOSFileWriter {
    fn write(&self, path: &str, content: &str) {
        println!("Writing to file in macOS file system at {}", path);
        println!("Content: {}", content);
    }
}

struct MacOSDirectoryHandler;

impl DirectoryHandler for MacOSDirectoryHandler {
    fn create_directory(&self, path: &str) {
        println!("Creating directory in macOS file system at {}", path);
    }
}

// Abstract Factory
trait FileSystemFactory {
    fn create_file_reader(&self) -> Box<dyn FileReader>;
    fn create_file_writer(&self) -> Box<dyn FileWriter>;
    fn create_directory_handler(&self) -> Box<dyn DirectoryHandler>;
}

// Concrete Factories

struct WindowsFactory;
impl FileSystemFactory for WindowsFactory {
    fn create_file_reader(&self) -> Box<dyn FileReader> {
        Box::new(WindowsFileReader)
    }

    fn create_file_writer(&self) -> Box<dyn FileWriter> {
        Box::new(WindowsFileWriter)
    }

    fn create_directory_handler(&self) -> Box<dyn DirectoryHandler> {
        Box::new(WindowsDirectoryHandler)
    }
}

struct LinuxFactory;

impl FileSystemFactory for LinuxFactory {
    fn create_file_reader(&self) -> Box<dyn FileReader> {
        Box::new(LinuxFileReader)
    }

    fn create_file_writer(&self) -> Box<dyn FileWriter> {
        Box::new(LinuxFileWriter)
    }

    fn create_directory_handler(&self) -> Box<dyn DirectoryHandler> {
        Box::new(LinuxDirectoryHandler)
    }
}

struct MacOSFactory;

impl FileSystemFactory for MacOSFactory {
    fn create_file_reader(&self) -> Box<dyn FileReader> {
        Box::new(MacOSFileReader)
    }

    fn create_file_writer(&self) -> Box<dyn FileWriter> {
        Box::new(MacOSFileWriter)
    }

    fn create_directory_handler(&self) -> Box<dyn DirectoryHandler> {
        Box::new(MacOSDirectoryHandler)
    }
}

// Client code
fn main() {
    let os_type = "Linux";

    let factory: Box<dyn FileSystemFactory> = match os_type {
        "Windows" => Box::new(WindowsFactory),
        "Linux" => Box::new(LinuxFactory),
        "macOS" => Box::new(MacOSFactory),
        _ => panic!("Unsupported OS type"),
    };

    let file_reader = factory.create_file_reader();
    let file_writer = factory.create_file_writer();
    let directory_handler = factory.create_directory_handler();

    let path = "/path/to/file";
    let content = file_reader.read(path);
    println!("Read content: {}", content);

    file_writer.write(path, "new");
    directory_handler.create_directory("/path/to/new_diẻctory");
}
