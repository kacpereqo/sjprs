struct HTMLParser {
    html: String,
}

struct HTMLNode {
    tag: String,
    children: Vec<HTMLNode>,
}

impl HTMLParser {
    fn new(html: String) -> HTMLParser {
        HTMLParser { html }
    }

    fn parse(&self) {
        println!("Parsing HTML: {}", self.html);
    }

    fn parse_node(&self, node: HTMLNode) {
        println!("Parsing node: {}", node.tag);
    }
}

fn main() {
    let sample = "<html><body><h1>Hello, World!</h1></body></html>";
    let parser = HTMLParser::new(sample.to_string());

    parser.parse();
}
