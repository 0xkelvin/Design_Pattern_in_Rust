// Define the Query Components and Product
#[derive(Debug)]
struct Query {
    table: String,
    columns: Vec<String>,
    conditions: Vec<String>,
    order_by: Option<String>,
}

// Define the Builder Trait
trait QueryBuilder {
    fn set_table(&mut self, table: &str) -> &mut Self;
    fn add_column(&mut self, column: &str) -> &mut Self;
    fn add_condition(&mut self, condition: &str) -> &mut Self;
    fn set_order_by(&mut self, order_by: &str) -> &mut Self;
    fn build(&self) -> Query;
}

// Implement the Concrete Builder
struct SqlQueryBuilder {
    table: String,
    columns: Vec<String>,
    conditions: Vec<String>,
    order_by: Option<String>,
}

impl SqlQueryBuilder {
    fn new() -> Self {
        SqlQueryBuilder {
            table: String::new(),
            columns: Vec::new(),
            conditions: Vec::new(),
            order_by: None,
        }
    }
}

impl QueryBuilder for SqlQueryBuilder {
    fn set_table(&mut self, table: &str) -> &mut Self {
        self.table = table.to_string();
        self
    }

    fn add_column(&mut self, column: &str) -> &mut Self {
        self.columns.push(column.to_string());
        self
    }

    fn add_condition(&mut self, condition: &str) -> &mut Self {
        self.conditions.push(condition.to_string());
        self
    }

    fn set_order_by(&mut self, order_by: &str) -> &mut Self {
        self.order_by = Some(order_by.to_string());
        self
    }

    fn build(&self) -> Query {
        Query {
            table: self.table.clone(),
            columns: self.columns.clone(),
            conditions: self.conditions.clone(),
            order_by: self.order_by.clone(),
        }
    }
}

fn main() {
    let mut builder = SqlQueryBuilder::new();

    let select_query = builder
        .set_table("users")
        .add_column("id")
        .add_column("name")
        .add_condition("age > 18")
        .set_order_by("name ASC")
        .build();

    let insert_query = builder
        .set_table("users")
        .add_column("id")
        .add_column("name")
        .add_column("email")
        .build();

    println!("SELECT Query: {:?}", select_query);
    println!("INSERT Query: {:?}", insert_query);
}
