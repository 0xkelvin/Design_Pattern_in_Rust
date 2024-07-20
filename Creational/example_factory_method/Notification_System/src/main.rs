// Define the Product trait
trait Notification {
    fn send(&self, recipient: &str, message: &str);
}

// ConcreteProduct: EmailNotification
struct EmailNotification;

impl Notification for EmailNotification {
    fn send(&self, recpient: &str, message: &str) {
        println!("Email sent to {}: {}", recpient, message);
    }
}

// ConcreteProduct: SMSNotification
struct SMSNotification;

impl Notification for SMSNotification {
    fn send(&self, recipient: &str, message: &str) {
        println!("SMS sent to {}: {}", recipient, message);
    }
}

// ConcreteProduct: PushNotification
struct PushNotification;

impl Notification for PushNotification {
    fn send(&self, recipient: &str, message: &str) {
        println!("Push notification sent to {}: {}", recipient, message);
    }
}


// Creator
trait NotificationFactory {
    fn create_notification(&self) -> Box<dyn Notification>;
}

// ConcreteCreator: EmailNotificationFactory
struct EmailNotificationFactory;

impl NotificationFactory for EmailNotificationFactory {
    fn create_notification(&self) -> Box<dyn Notification> {
        Box::new(EmailNotification)
    }
}

// ConcreteCreator: SMSNotificationFactory
struct SMSNotificationFactory;

impl NotificationFactory for SMSNotificationFactory {
    fn create_notification(&self) -> Box<dyn Notification> {
        Box::new(SMSNotification)
    }
}

// ConcreteCreator: PushNotificationFactory
struct PushNotificationFactory;

impl NotificationFactory for PushNotificationFactory {
    fn create_notification(&self) -> Box<dyn Notification> {
        Box::new(PushNotification)
    }
}

// Client code
fn main() {
    let email_factory = EmailNotificationFactory;
    let sms_factory = SMSNotificationFactory;
    let push_factory = PushNotificationFactory;

    let notifications: Vec<Box<dyn Notification>> = vec![
        email_factory.create_notification(),
        sms_factory.create_notification(),
        push_factory.create_notification(),
    ];

    for notification in notifications {
        notification.send("user@example.com", "This is a notification message.");
    }
}
