// Abstract Products
trait ConnectionHandler {
    fn handle_connection(&self);
}

trait MessageProtocol {
    fn send_message(&self, message: &str);
    fn receive_message(&self) -> String;
}

trait DataProcessor {
    fn process_data(&self, data: &str);
}

// Concrete Products for File Sharing Peer
struct FileSharingConnectionHandler;

impl ConnectionHandler for FileSharingConnectionHandler {
    fn handle_connection(&self) {
        println!("Handling file sharing connection");
    }
}

struct FileSharingMessageProtocol;

impl MessageProtocol for FileSharingMessageProtocol {
    fn send_message(&self, message: &str) {
        println!("Sending file sharing message: {}", message);
    }

    fn receive_message(&self) -> String {
        println!("Receiving file sharing message");
        "File sharing message".to_string()
    }
}

struct FileSharingDataProcessor;

impl DataProcessor for FileSharingDataProcessor {
    fn process_data(&self, data: &str) {
        println!("Processing file sharing data: {data}")
    }
}

// Concrete Products for Chat Peer
struct ChatConnectionHandler;

impl ConnectionHandler for ChatConnectionHandler {
    fn handle_connection(&self) {
        println!("Handling chat connection");
    }
}

struct ChatMessageProtocol;

impl MessageProtocol for ChatMessageProtocol {
    fn send_message(&self, message: &str) {
        println!("Sending chat message: {}", message);
    }

    fn receive_message(&self) -> String {
        println!("Receiving chat message");
        "Chat message".to_string()
    }
}

struct ChatDataProcessor;

impl DataProcessor for ChatDataProcessor {
    fn process_data(&self, data: &str) {
        println!("Processing chat data: {}", data);
    }
}

// Concrete Products for Streaming Peer
struct StreamingConnectionHandler;
impl ConnectionHandler for StreamingConnectionHandler {
    fn handle_connection(&self) {
        println!("Handling streaming connection");
    }
}

struct StreamingMessageProtocol;
impl MessageProtocol for StreamingMessageProtocol {
    fn send_message(&self, message: &str) {
        println!("Sending streaming message: {}", message)
    }

    fn receive_message(&self) -> String {
        println!("Receiving streaming message");
        "Streaming message".to_string()
    }
}

struct StreamingDataProcessor;

impl DataProcessor for StreamingDataProcessor {
    fn process_data(&self, data: &str) {
        println!("Processing streaming data: {}", data);
    }
}

// Abstract Factory
trait PeerFactory {
    fn create_connection_handler(&self) -> Box<dyn ConnectionHandler>;
    fn create_message_protocol(&self) -> Box<dyn MessageProtocol>;
    fn create_data_processor(&self) -> Box<dyn DataProcessor>;
}

// Concrete Factories
struct FileSharingPeerFactory;

impl PeerFactory for FileSharingPeerFactory {
    fn create_connection_handler(&self) -> Box<dyn ConnectionHandler> {
        Box::new(FileSharingConnectionHandler)
    }

    fn create_message_protocol(&self) -> Box<dyn MessageProtocol> {
        Box::new(FileSharingMessageProtocol)
    }

    fn create_data_processor(&self) -> Box<dyn DataProcessor> {
        Box::new(FileSharingDataProcessor)
    }
}

struct ChatPeerFactory;

impl PeerFactory for ChatPeerFactory {
    fn create_connection_handler(&self) -> Box<dyn ConnectionHandler> {
        Box::new(ChatConnectionHandler)
    }

    fn create_message_protocol(&self) -> Box<dyn MessageProtocol> {
        Box::new(ChatMessageProtocol)
    }

    fn create_data_processor(&self) -> Box<dyn DataProcessor> {
        Box::new(ChatDataProcessor)
    }
}

struct StreamingPeerFactory;

impl PeerFactory for StreamingPeerFactory {
    fn create_connection_handler(&self) -> Box<dyn ConnectionHandler> {
        Box::new(StreamingConnectionHandler)
    }

    fn create_message_protocol(&self) -> Box<dyn MessageProtocol> {
        Box::new(StreamingMessageProtocol)
    }

    fn create_data_processor(&self) -> Box<dyn DataProcessor> {
        Box::new(StreamingDataProcessor)
    }
}

// Client code
fn main() {
    let peer_type = "Chat";

    let factory: Box<dyn PeerFactory> = match peer_type {
        "FileSharing" => Box::new(FileSharingPeerFactory),
        "Chat" => Box::new(ChatPeerFactory),
        "Streaming" => Box::new(StreamingPeerFactory),
        _ => panic!("Unsupported peer type"),
    };

    let connection_handler = factory.create_connection_handler();
    let message_protocol = factory.create_message_protocol();
    let data_processor = factory.create_data_processor();

    connection_handler.handle_connection();
    message_protocol.send_message("Hello, world");
    let message = message_protocol.receive_message();
    println!("Received message: {}", message);
    data_processor.process_data("Some data");
}
