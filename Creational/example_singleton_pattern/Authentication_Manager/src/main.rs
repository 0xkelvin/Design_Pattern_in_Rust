use std::collections::HashMap;
use std::sync::Once;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

struct AuthenticationManager {
    user_sessions: Mutex<HashMap<String, String>>, // token -> user_id
}

impl AuthenticationManager {
    fn new() -> Arc<AuthenticationManager> {
        static mut SINGLETON: Option<Arc<AuthenticationManager>> = None;
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                let auth_manager = AuthenticationManager {
                    user_sessions: Mutex::new(HashMap::new()),
                };
                SINGLETON = Some(Arc::new(auth_manager));
            });

            SINGLETON.clone().unwrap()
        }
    }

    fn create_session(&self, user_id: &str) -> String {
        let token = Uuid::new_v4().to_string();
        let mut sessions = self.user_sessions.lock().unwrap();
        sessions.insert(token.clone(), user_id.to_string());
        token
    }

    fn validate_token(&self, token: &str) -> bool {
        let sessions = self.user_sessions.lock().unwrap();
        sessions.contains_key(token)
    }

    fn get_user_id(&self, token: &str) -> Option<String> {
        let sessions = self.user_sessions.lock().unwrap();
        sessions.get(token).cloned()
    }
}

fn main() {
    let auth_manager = AuthenticationManager::new();

    // create a session for a user
    let user_id = "user123";
    let token = auth_manager.create_session(user_id);
    println!("Created session for user {}: token {}", user_id, token);

    // Validate the token
    if auth_manager.validate_token(&token) {
        println!("Token is valid");
        let retrieved_user_id = auth_manager.get_user_id(&token).unwrap();
        println!("User ID for token: {}", retrieved_user_id);
    } else {
        println!("Token is invalid");
    }

    // Use the singleton instance in another part of the application
    let another = AuthenticationManager::new();
    let another_token = another.create_session("user456");
    println!("Created session for another user: token {}", another_token);
}
