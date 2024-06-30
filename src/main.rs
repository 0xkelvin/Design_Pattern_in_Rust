// Define the Product trait
trait DatabaseConnection {
    fn connect(&self);
    fn execute_query(&self, query: &str);
}

// Concrete Product: MySQL Connection
struct MySqlConnection;

impl DatabaseConnection for MySqlConnection {
    fn connect(&self) {
        println!("Connecting to MySQL database...");
    }

    fn execute_query(&self, query: &str) {
        println!("Executing query: {}", query);
    }
}


// Concrete Product: PostgreSQL Connection
struct PostgreSQLConnection;

impl DatabaseConnection for PostgreSQLConnection {
    fn connect(&self) {
        println!("Connecting to PostgreSQL database...");
    }

    fn execute_query(&self, query: &str) {
        println!("Executing query: {}", query);
    }
}

// ConcreteProduct: SQLiteConnection
struct SQLiteConnection;

impl DatabaseConnection for SQLiteConnection {
    fn connect(&self) {
        println!("Connecting to SQLite database...");
    }

    fn execute_query(&self, query: &str) {
        println!("Executing query: {}", query);
    }
}


// Creator 
trait ConnectionFactory {
    fn create_connection(&self) -> Box<dyn DatabaseConnection>;
}

// ConcreteCreator: MySQLConnectionFactory
struct MySQLConnectionFactory;

impl ConnectionFactory for MySQLConnectionFactory {
    fn create_connection(&self) -> Box<dyn DatabaseConnection> {
        Box::new(MySqlConnection)
    }
}


// ConcreteCreator: PostgreSQLConnectionFactory
struct PostgreSQLConnectionFactory;

impl ConnectionFactory for PostgreSQLConnectionFactory {
    fn create_connection(&self) -> Box<dyn DatabaseConnection> {
        Box::new(PostgreSQLConnection)
    }
}

// ConcreteCreator: SQLiteConnectionFactory

struct SQLiteConnectionFactory;

impl ConnectionFactory for SQLiteConnectionFactory {
    fn create_connection(&self) -> Box<dyn DatabaseConnection> {
        Box::new(SQLiteConnection)
    }
}

// Client code 

fn main() {
    let mysql_factory = MySQLConnectionFactory;
    let mysql_connection = mysql_factory.create_connection();
    mysql_connection.connect();
    mysql_connection.execute_query("SELECT * FROM users");

    let postgresql_factory = PostgreSQLConnectionFactory;
    let postgresql_connection = postgresql_factory.create_connection();
    postgresql_connection.connect();
    postgresql_connection.execute_query("SELECT * FROM users");

    let sqlite_factory = SQLiteConnectionFactory;
    let sqlite_connection = sqlite_factory.create_connection();
    sqlite_connection.connect();
    sqlite_connection.execute_query("SELECT * FROM users");
}
