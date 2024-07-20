use std::collections::VecDeque;
use std::sync::Once;
use std::sync::{Arc, Mutex};

struct DbConnection {
    // Mock structure for a database connection
}

struct ConnectionPool {
    connections: Mutex<VecDeque<DbConnection>>,
}

impl ConnectionPool {
    fn new() -> Arc<ConnectionPool> {
        static mut SINGLETON: Option<Arc<ConnectionPool>> = None;
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                let pool = ConnectionPool {
                    connections: Mutex::new(VecDeque::new()),
                };
                SINGLETON = Some(Arc::new(pool));
            });
            SINGLETON.clone().unwrap()
        }
    }

    fn get_connection(&self) -> Option<DbConnection> {
        let mut connections = self.connections.lock().unwrap();
        connections.pop_front()
    }

    fn return_connection(&self, conn: DbConnection) {
        let mut connections = self.connections.lock().unwrap();
        connections.push_back(conn);
    }

    fn add_connection(&self, conn: DbConnection) {
        let mut connections = self.connections.lock().unwrap();
        connections.push_back(conn);
    }
}

fn main() {
    let pool = ConnectionPool::new();

    // Simulate adding connectionis to the pool
    pool.add_connection(DbConnection {});
    pool.add_connection(DbConnection {});

    // Simulate getting a connection from the pool
    if let Some(conn) = pool.get_connection() {
        println!("Got a connection from the pool");
        // Use the connection...
        pool.return_connection(conn);
    } else {
        println!("No available connections");
    }
}
