use std::fs::OpenOptions;

// Define the Request Components and Product
#[derive(Debug)]
struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    query_params: Vec<(String, String)>,
    body: Option<String>,
}

// Define the Builder Trait
trait HttpRequestBuilder {
    fn set_method(&mut self, method: &str) -> &mut Self;
    fn set_url(&mut self, url: &str) -> &mut Self;
    fn add_header(&mut self, key: &str, value: &str) -> &mut Self;
    fn add_query_param(&mut self, key: &str, value: &str) -> &mut Self;
    fn set_body(&mut self, body: &str) -> &mut Self;
    fn build(&self) -> HttpRequest;
}

// Implement the Concrete Builder
struct CustomHttpRequestBuilder {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    query_params: Vec<(String, String)>,
    body: Option<String>,
}

impl CustomHttpRequestBuilder {
    fn new() -> Self {
        CustomHttpRequestBuilder {
            method: String::new(),
            url: String::new(),
            headers: Vec::new(),
            query_params: Vec::new(),
            body: None,
        }
    }
}

impl HttpRequestBuilder for CustomHttpRequestBuilder {
    fn set_method(&mut self, method: &str) -> &mut Self {
        self.method = method.to_string();
        self
    }

    fn set_url(&mut self, url: &str) -> &mut Self {
        self.url = url.to_string();
        self
    }

    fn add_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    fn add_query_param(&mut self, key: &str, value: &str) -> &mut Self {
        self.query_params.push((key.to_string(), value.to_string()));
        self
    }

    fn set_body(&mut self, body: &str) -> &mut Self {
        self.body = Some(body.to_string());
        self
    }

    fn build(&self) -> HttpRequest {
        HttpRequest {
            method: self.method.clone(),
            url: self.url.clone(),
            headers: self.headers.clone(),
            query_params: self.query_params.clone(),
            body: self.body.clone(),
        }
    }
}

fn main() {
    let mut builder = CustomHttpRequestBuilder::new();

    let get_request = builder
        .set_method("GET")
        .set_url("https://api.example.com/resource")
        .add_query_param("key", "value")
        .add_header("Accept", "application/json")
        .build();

    let post_request = builder
        .set_method("POST")
        .set_url("https://api.example.com/resource")
        .add_header("Content-Type", "application/json")
        .set_body("{\"key\": \"value\"}")
        .build();

    println!("GET Request: {:?}", get_request);
    println!("POST Request: {:?}", post_request);
}
