// Define the Component Trait
trait FileIO {
    fn read(&mut self) -> String;
    fn write(&mut self, data: &str);
}

// implement Concrete Components
struct BasicFileIO {
    filename: String,
}

impl BasicFileIO {
    fn new(filename: &str) -> Self {
        BasicFileIO {
            filename: filename.to_string(),
        }
    }
}

impl FileIO for BasicFileIO {
    fn read(&mut self) -> String {
        println!("Reading from file: {}", self.filename);
        "file data".to_string()
    }

    fn write(&mut self, data: &str) {
        println!("Writing {} to file: {}", data, self.filename);
    }
}

// Implement the Decorator Structs
struct BufferedFileIO {
    file_io: Box<dyn FileIO>,
    buffer: String,
}

impl BufferedFileIO {
    fn new(file_io: Box<dyn FileIO>) -> Self {
        BufferedFileIO {
            file_io,
            buffer: String::new(),
        }
    }
}

impl FileIO for BufferedFileIO {
    fn read(&mut self) -> String {
        if self.buffer.is_empty() {
            self.buffer = self.file_io.read();
        }
        println!("Reading from buffer");
        self.buffer.clone()
    }

    fn write(&mut self, data: &str) {
        self.buffer = data.to_string();
        println!("Writing to buffer");
        self.file_io.write(data);
    }
}

struct EncryptedFileIO {
    file_io: Box<dyn FileIO>,
}

impl EncryptedFileIO {
    fn new(file_io: Box<dyn FileIO>) -> Self {
        EncryptedFileIO { file_io }
    }

    fn encrypt(&self, data: &str) -> String {
        format!("encrypted({})", data)
    }

    fn decrypt(&self, data: &str) -> String {
        data.replace("encrypted(", "").replace(")", "")
    }
}

impl FileIO for EncryptedFileIO {
    fn read(&mut self) -> String {
        let encrypted_data = self.file_io.read();
        self.decrypt(&encrypted_data)
    }

    fn write(&mut self, data: &str) {
        let encrypted_data = self.encrypt(data);
        self.file_io.write(&encrypted_data);
    }
}

struct CompressedFileIO {
    file_io: Box<dyn FileIO>,
}

impl CompressedFileIO {
    fn new(file_io: Box<dyn FileIO>) -> Self {
        CompressedFileIO { file_io }
    }

    fn compress(&self, data: &str) -> String {
        // Simulate compression
        format!("compressed({})", data)
    }

    fn decompress(&self, data: &str) -> String {
        // Simulate decompression
        data.replace("compressed(", "").replace(")", "")
    }
}

impl FileIO for CompressedFileIO {
    fn read(&mut self) -> String {
        let compressed_data = self.file_io.read();
        self.decompress(&compressed_data)
    }

    fn write(&mut self, data: &str) {
        let compressed_data = self.compress(data);
        self.file_io.write(&compressed_data);
    }
}

fn main() {
    let mut basic_file_io: Box<dyn FileIO> = Box::new(BasicFileIO::new("file.txt"));
    println!("BasicFileIO:");
    basic_file_io.write("hello");
    println!("Read: {}", basic_file_io.read());

    let mut buffered_file_io: Box<dyn FileIO> = Box::new(BufferedFileIO::new(basic_file_io));
    println!("\nBufferedFileIO:");
    buffered_file_io.write("hello buffered");
    println!("Read: {}", buffered_file_io.read());

    let mut encrypted_file_io: Box<dyn FileIO> = Box::new(EncryptedFileIO::new(buffered_file_io));
    println!("\nEncryptedFileIO:");
    encrypted_file_io.write("hello encrypted");
    println!("Read: {}", encrypted_file_io.read());

    let mut compressed_file_io: Box<dyn FileIO> =
        Box::new(CompressedFileIO::new(encrypted_file_io));
    println!("\nCompressedFileIO:");
    compressed_file_io.write("hello compressed");
    println!("Read: {}", compressed_file_io.read());
}
