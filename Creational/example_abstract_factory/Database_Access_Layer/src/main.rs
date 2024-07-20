// Abstract Products
trait DBConnection {
    fn connect(&self);
}

trait DBCommand {
    fn execute(&self, query: &str);
}

trait DBReader {
    fn read(&self) -> Vec<String>;
}

// Concrete Products for SQL Server
struct SQLServerConnection;

impl DBConnection for SQLServerConnection {
    fn connect(&self) {
        println!("Connecting to SQL Server");
    }
}

struct SQLServerCommand;

impl DBCommand for SQLServerCommand {
    fn execute(&self, query: &str) {
        println!("Executing SQL Server query: {}", query);
    }
}

struct SQLServerReader;

impl DBReader for SQLServerReader {
    fn read(&self) -> Vec<String> {
        vec!["SQL Server data".to_string()]
    }
}

// Concrete Products for MySQL
struct MySQLConnection;

impl DBConnection for MySQLConnection {
    fn connect(&self) {
        println!("Connecting to MySQL");
    }
}

struct MySQLCommand;

impl DBCommand for MySQLCommand {
    fn execute(&self, query: &str) {
        println!("Executing MySQL query: {}", query);
    }
}

struct MySQLReader;

impl DBReader for MySQLReader {
    fn read(&self) -> Vec<String> {
        vec!["MySQL data".to_string()]
    }
}

struct PostgreSQLConnection;

impl DBConnection for PostgreSQLConnection {
    fn connect(&self) {
        println!("Connecting to PostgreSQL");
    }
}

struct PostgreSQLCommand;

impl DBCommand for PostgreSQLCommand {
    fn execute(&self, query: &str) {
        println!("Executing PostgreSQL query: {}", query);
    }
}

struct PostgreSQLReader;

impl DBReader for PostgreSQLReader {
    fn read(&self) -> Vec<String> {
        vec!["PostgreSQL data".to_string()]
    }
}

// Abstract Factory
trait DBFactory {
    fn create_connection(&self) -> Box<dyn DBConnection>;
    fn create_command(&self) -> Box<dyn DBCommand>;
    fn create_reader(&self) -> Box<dyn DBReader>;
}

struct SQLServerFactory;

impl DBFactory for SQLServerFactory {
    fn create_connection(&self) -> Box<dyn DBConnection> {
        Box::new(SQLServerConnection)
    }

    fn create_command(&self) -> Box<dyn DBCommand> {
        Box::new(SQLServerCommand)
    }

    fn create_reader(&self) -> Box<dyn DBReader> {
        Box::new(SQLServerReader)
    }
}

struct MySQLFactory;

impl DBFactory for MySQLFactory {
    fn create_connection(&self) -> Box<dyn DBConnection> {
        Box::new(MySQLConnection)
    }

    fn create_command(&self) -> Box<dyn DBCommand> {
        Box::new(MySQLCommand)
    }

    fn create_reader(&self) -> Box<dyn DBReader> {
        Box::new(MySQLReader)
    }
}

struct PostgreSQLFactory;

impl DBFactory for PostgreSQLFactory {
    fn create_connection(&self) -> Box<dyn DBConnection> {
        Box::new(PostgreSQLConnection)
    }

    fn create_command(&self) -> Box<dyn DBCommand> {
        Box::new(PostgreSQLCommand)
    }

    fn create_reader(&self) -> Box<dyn DBReader> {
        Box::new(PostgreSQLReader)
    }
}

//Client Code
fn main() {
    let db_type = "PostgreSQL"; // This can be dynamic based on runtime conditions
    let factory: Box<dyn DBFactory> = match db_type {
        "SQLServer" => Box::new(SQLServerFactory),
        "MySQL" => Box::new(MySQLFactory),
        "PostgreSQL" => Box::new(PostgreSQLFactory),
        _ => panic!("Unsupported DB type"),
    };

    let connection = factory.create_connection();
    let command = factory.create_command();
    let reader = factory.create_reader();

    connection.connect();
    command.execute("SELECT * FROM users");
    let data = reader.read();

    println!("Data: {:?}", data);
}
