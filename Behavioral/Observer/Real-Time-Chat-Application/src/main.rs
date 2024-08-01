// Define the Subject and Observer Traits
use std::cell::RefCell;
use std::rc::Rc;

trait Observer {
    fn update(&self, message: &str);
}

trait Subject {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn notify_observers(&self, message: &str);
}

// Implement the ChatRoom Struct as the Subject
struct ChatRoom {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    messages: Vec<String>,
}

impl ChatRoom {
    fn new() -> Self {
        ChatRoom {
            observers: Vec::new(),
            messages: Vec::new(),
        }
    }

    fn post_message(&mut self, message: &str) {
        self.messages.push(message.to_string());
        self.notify_observers(message)
    }
}

impl Subject for ChatRoom {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.retain(|obs| !Rc::ptr_eq(obs, &observer));
    }

    fn notify_observers(&self, message: &str) {
        for observer in &self.observers {
            observer.borrow().update(message);
        }
    }
}

// Implement Concrete Observers
struct TextUser {
    username: String,
}

impl TextUser {
    fn new(username: &str) -> Self {
        TextUser {
            username: username.to_string(),
        }
    }

    fn display_message(&self, message: &str) {
        println!("{} (text): {}", self.username, message);
    }
}

impl Observer for TextUser {
    fn update(&self, message: &str) {
        self.display_message(message);
    }
}

struct EmoticonUser {
    username: String,
}

impl EmoticonUser {
    fn new(username: &str) -> Self {
        EmoticonUser {
            username: username.to_string(),
        }
    }

    fn display_message(&self, message: &str) {
        let emoticon_message = message.replace(":)", "😊").replace(":(", "😢");
        println!("{} (emoticon): {}", self.username, emoticon_message);
    }
}

impl Observer for EmoticonUser {
    fn update(&self, message: &str) {
        self.display_message(message);
    }
}

struct ThemedUser {
    username: String,
    theme: String,
}

impl ThemedUser {
    fn new(username: &str, theme: &str) -> Self {
        ThemedUser {
            username: username.to_string(),
            theme: theme.to_string(),
        }
    }

    fn display_message(&self, message: &str) {
        println!("{} ({} theme): {}", self.username, self.theme, message);
    }
}

impl Observer for ThemedUser {
    fn update(&self, message: &str) {
        self.display_message(message);
    }
}

// Use the Observer Pattern in the Client Code
fn main() {
    let chat_room = Rc::new(RefCell::new(ChatRoom::new()));

    let text_user = Rc::new(RefCell::new(TextUser::new("Alice")));
    let emoticon_user = Rc::new(RefCell::new(EmoticonUser::new("Bob")));
    let themed_user = Rc::new(RefCell::new(ThemedUser::new("Charlie", "Dark")));

    chat_room.borrow_mut().register_observer(text_user.clone());
    chat_room
        .borrow_mut()
        .register_observer(emoticon_user.clone());
    chat_room
        .borrow_mut()
        .register_observer(themed_user.clone());

    chat_room.borrow_mut().post_message("Hello, everyone! :)");
    chat_room.borrow_mut().post_message("How's it going? :(");

    chat_room.borrow_mut().remove_observer(text_user.clone());
    chat_room
        .borrow_mut()
        .post_message("Alice has left the chat.");
}
