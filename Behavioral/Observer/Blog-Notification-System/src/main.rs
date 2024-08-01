//Define the Subject and Observer Traits
use std::cell::RefCell;
use std::rc::Rc;

trait Observer {
    fn update(&self, blog_title: &str);
}

trait Subject {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn notify_observers(&mut self, blog_title: &str);
}

// Implement the Blog Struct as the Subject
struct Blog {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    posts: Vec<String>,
}

impl Blog {
    fn new() -> Self {
        Blog {
            observers: Vec::new(),
            posts: Vec::new(),
        }
    }

    fn add_post(&mut self, title: &str) {
        self.posts.push(title.to_string());
        self.notify_observers(title);
    }
}

impl Subject for Blog {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.retain(|obs| !Rc::ptr_eq(obs, &observer));
    }

    fn notify_observers(&mut self, blog_title: &str) {
        for observer in &self.observers {
            observer.borrow_mut().update(blog_title);
        }
    }
}

// Implement Concrete Observers
struct EmailSubscriber {
    email: String,
}

impl EmailSubscriber {
    fn new(email: &str) -> Self {
        EmailSubscriber {
            email: email.to_string(),
        }
    }

    fn send_email(&self, blog_title: &str) {
        println!(
            "Sending email to {} about new post: {}",
            self.email, blog_title
        );
    }
}

impl Observer for EmailSubscriber {
    fn update(&self, blog_title: &str) {
        self.send_email(blog_title);
    }
}

struct SmsSubscriber {
    phone_number: String,
}

impl SmsSubscriber {
    fn new(phone_number: &str) -> Self {
        SmsSubscriber {
            phone_number: phone_number.to_string(),
        }
    }

    fn send_sms(&self, blog_title: &str) {
        println!(
            "Sending SMS to {} about new post: {}",
            self.phone_number, blog_title
        );
    }
}

impl Observer for SmsSubscriber {
    fn update(&self, blog_title: &str) {
        self.send_sms(blog_title);
    }
}

struct PushNotificationSubscriber {
    device_id: String,
}

impl PushNotificationSubscriber {
    fn new(device_id: &str) -> Self {
        PushNotificationSubscriber {
            device_id: device_id.to_string(),
        }
    }

    fn send_push_notification(&self, blog_title: &str) {
        println!(
            "Sending push notification to device {} about new post: {}",
            self.device_id, blog_title
        );
    }
}

impl Observer for PushNotificationSubscriber {
    fn update(&self, blog_title: &str) {
        self.send_push_notification(blog_title);
    }
}

fn main() {
    let blog = Rc::new(RefCell::new(Blog::new()));

    let email_subscriber = Rc::new(RefCell::new(EmailSubscriber::new("subscrible@example.com")));
    let sms_subscriber = Rc::new(RefCell::new(SmsSubscriber::new("+123456789")));
    let push_subscriber = Rc::new(RefCell::new(PushNotificationSubscriber::new("device123")));

    blog.borrow_mut()
        .register_observer(email_subscriber.clone());
    blog.borrow_mut().register_observer(sms_subscriber.clone());
    blog.borrow_mut().register_observer(push_subscriber.clone());

    blog.borrow_mut().add_post("Observer Pattern in Rust");
    blog.borrow_mut().add_post("Understanding Rust Ownership");

    blog.borrow_mut().remove_observer(email_subscriber.clone());
    blog.borrow_mut().add_post("Advanced Rust Programming");
}
