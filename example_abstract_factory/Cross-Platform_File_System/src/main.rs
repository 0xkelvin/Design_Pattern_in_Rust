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
