use std::collections::HashMap;

// Define the Product trait
trait Content {
    fn create(&self, title: &str, body: &str);
    fn display(&self);
    fn metadata(&self) -> HashMap<String, String>;
}

// ConcreteProduct: BlogPost
struct BlogPost {
    title: String,
    body: String,
    tags: Vec<String>,
}

impl BlogPost {
    fn new(title: &str, body: &str) -> BlogPost {
        BlogPost {
            title: title.to_string(),
            body: body.to_string(),
            tags: vec![],
        }
    }
}

impl Content for BlogPost {
    fn create(&self, title: &str, body: &str) {
        println!("Creating Blog Post: {} - {}", title, body);
    }

    fn display(&self) {
        println!("Displaying Blog Post: {} - {}", self.title, self.body);
    }

    fn metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "BlogPost".to_string());
        metadata.insert("title".to_string(), self.title.clone());
        metadata.insert("body".to_string(), self.body.clone());
        metadata
    }
}

// ConcreteProduct: Video
struct Video {
    title: String,
    url: String,
    duration: u32,
}

impl Video {
    fn new(title: &str, url: &str, duration: u32) -> Video {
        Video {
            title: title.to_string(),
            url: url.to_string(),
            duration,
        }
    }
}

impl Content for Video {
    fn create(&self, title: &str, body: &str) {
        println!("Creating Video: {} - {}", title, body);
    }

    fn display(&self) {
        println!(
            "Displaying video: {}\nURL: {}\nDuration: {} seconds",
            self.title, self.url, self.duration
        );
    }

    fn metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "Video".to_string());
        metadata.insert("title".to_string(), self.title.clone());
        metadata.insert("url".to_string(), self.url.clone());
        metadata.insert("duration".to_string(), self.duration.to_string());
        metadata
    }
}

// Creator
trait ContentFactory {
    fn create_content(&self, title: &str, body: &str) -> Box<dyn Content>;
}

// ConcreteCreator: BlogPostFactory
struct BlogPostFactory;

impl ContentFactory for BlogPostFactory {
    fn create_content(&self, title: &str, body: &str) -> Box<dyn Content> {
        Box::new(BlogPost::new(title, body))
    }
}

// ConcreteCreator: VideoFactory
struct VideoFactory;

impl ContentFactory for VideoFactory {
    fn create_content(&self, title: &str, body: &str) -> Box<dyn Content> {
        let video_url = "https://www.youtube.com/";
        Box::new(Video::new(title, video_url, 60))
    }
}

// Client code
fn main() {
    let blog_post_factory = BlogPostFactory;
    let video_factory = VideoFactory;

    let blog_post = blog_post_factory.create_content("Design Patterns", "Factory Method Pattern");
    let video = video_factory.create_content("Design Patterns", "Factory Method Pattern");

    blog_post.create("Design Patterns", "Factory Method Pattern");
    blog_post.display();
    println!("{:?}", blog_post.metadata());

    video.create("Design Patterns", "Factory Method Pattern");
    video.display();
    println!("{:?}", video.metadata());
}
