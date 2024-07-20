use std::rc;

// Abstract Products
trait NotificationSender {
    fn send(&self, message: &str);
}

trait MessageFormatter {
    fn format(&self, message: &str) -> String;
}

trait NotificationLogger {
    fn log(&self, message: &str);
}

// Concrete Product for Email Notification
struct EmailNotificationSender;

impl NotificationSender for EmailNotificationSender {
    fn send(&self, message: &str) {
        println!("Sending email notification: {}", message);
    }
}

struct EmailMessageFormatter;

impl MessageFormatter for EmailMessageFormatter {
    fn format(&self, message: &str) -> String {
        format!("Email formatted message: {}", message)
    }
}

struct EmailNotificationLogger;

impl NotificationLogger for EmailNotificationLogger {
    fn log(&self, message: &str) {
        println!("Logging email notification: {}", message)
    }
}

// Concrete Products for SMS Notification
struct SMSNotificationSender;

impl NotificationSender for SMSNotificationSender {
    fn send(&self, message: &str) {
        println!("Sending SMS notification: {}", message)
    }
}

struct SMSMessageFormatter;

impl MessageFormatter for SMSMessageFormatter {
    fn format(&self, message: &str) -> String {
        format!("SMS formatted message: {}", message)
    }
}

struct SMSNotificationLogger;

impl NotificationLogger for SMSNotificationLogger {
    fn log(&self, message: &str) {
        println!("Logging SMS notification: {}", message)
    }
}

// Concrete Products for Push Notification
struct PushNotificationSender;

impl NotificationSender for PushNotificationSender {
    fn send(&self, message: &str) {
        println!("Sending push notification: {}", message);
    }
}

struct PushMessageFormatter;

impl MessageFormatter for PushMessageFormatter {
    fn format(&self, message: &str) -> String {
        format!("Push formatted message: {}", message)
    }
}

struct PushNotificationLogger;

impl NotificationLogger for PushNotificationLogger {
    fn log(&self, message: &str) {
        println!("Logging push notification: {}", message);
    }
}

// Abstract Factory
trait NotificationFactory {
    fn create_sender(&self) -> Box<dyn NotificationSender>;
    fn create_formatter(&self) -> Box<dyn MessageFormatter>;
    fn create_logger(&self) -> Box<dyn NotificationLogger>;
}

// Concrete Factories
struct EmailNotificationFactory;

impl NotificationFactory for EmailNotificationFactory {
    fn create_sender(&self) -> Box<dyn NotificationSender> {
        Box::new(EmailNotificationSender)
    }

    fn create_formatter(&self) -> Box<dyn MessageFormatter> {
        Box::new(EmailMessageFormatter)
    }

    fn create_logger(&self) -> Box<dyn NotificationLogger> {
        Box::new(EmailNotificationLogger)
    }
}

struct SMSNotificationFactory;

impl NotificationFactory for SMSNotificationFactory {
    fn create_sender(&self) -> Box<dyn NotificationSender> {
        Box::new(SMSNotificationSender)
    }

    fn create_formatter(&self) -> Box<dyn MessageFormatter> {
        Box::new(SMSMessageFormatter)
    }

    fn create_logger(&self) -> Box<dyn NotificationLogger> {
        Box::new(SMSNotificationLogger)
    }
}

struct PushNotificationFactory;

impl NotificationFactory for PushNotificationFactory {
    fn create_sender(&self) -> Box<dyn NotificationSender> {
        Box::new(PushNotificationSender)
    }

    fn create_formatter(&self) -> Box<dyn MessageFormatter> {
        Box::new(PushMessageFormatter)
    }

    fn create_logger(&self) -> Box<dyn NotificationLogger> {
        Box::new(PushNotificationLogger)
    }
}

//CLient code
fn main() {
    let notification_type = "Push";

    let factory: Box<dyn NotificationFactory> = match notification_type {
        "Email" => Box::new(EmailNotificationFactory),
        "SMS" => Box::new(SMSNotificationFactory),
        "Push" => Box::new(PushNotificationFactory),
        _ => panic!("Unsupported notification type"),
    };

    let send = factory.create_sender();
    let formatter = factory.create_formatter();
    let logger = factory.create_logger();

    let raw_message = "Hello, this is a notification";
    let formatted_message = formatter.format(raw_message);

    send.send(&formatted_message);
    logger.log(&formatted_message);
}
