// Define the Component Trait
trait TextProcessor {
    fn process(&self, text: &str) -> String;
}

// Implement Conrete Components
struct PlainTextProcessor;

impl TextProcessor for PlainTextProcessor {
    fn process(&self, text: &str) -> String {
        text.to_string()
    }
}

// Implement the Decorator Structs
struct TrimDecorator {
    processor: Box<dyn TextProcessor>,
}

impl TrimDecorator {
    fn new(processor: Box<dyn TextProcessor>) -> Self {
        TrimDecorator { processor }
    }
}

impl TextProcessor for TrimDecorator {
    fn process(&self, text: &str) -> String {
        self.processor.process(text).trim().to_string()
    }
}

struct UpperCaseDecorator {
    processor: Box<dyn TextProcessor>,
}

impl UpperCaseDecorator {
    fn new(processor: Box<dyn TextProcessor>) -> Self {
        UpperCaseDecorator { processor }
    }
}

impl TextProcessor for UpperCaseDecorator {
    fn process(&self, text: &str) -> String {
        self.processor.process(text).to_uppercase()
    }
}

struct HtmlTagDecorator {
    processor: Box<dyn TextProcessor>,
    tag: String,
}

impl HtmlTagDecorator {
    fn new(processor: Box<dyn TextProcessor>, tag: &str) -> Self {
        HtmlTagDecorator {
            processor,
            tag: tag.to_string(),
        }
    }
}

impl TextProcessor for HtmlTagDecorator {
    fn process(&self, text: &str) -> String {
        let processed_text = self.processor.process(text);
        format!(
            "<{tag}>{text}</{tag}>",
            tag = self.tag,
            text = processed_text
        )
    }
}

fn main() {
    // Create a plain text processor
    let plain_text_processor: Box<dyn TextProcessor> = Box::new(PlainTextProcessor);

    // Apply decorators
    let trimmed_processor: Box<dyn TextProcessor> =
        Box::new(TrimDecorator::new(plain_text_processor));
    let uppercased_processor: Box<dyn TextProcessor> =
        Box::new(UpperCaseDecorator::new(trimmed_processor));
    let html_tagged_processor: Box<dyn TextProcessor> =
        Box::new(HtmlTagDecorator::new(uppercased_processor, "b"));

    // Process text with all decorators
    let input_text = "  hello, world!  ";
    let processed_text = html_tagged_processor.process(input_text);
    println!("Processed text: {}", processed_text);
}
