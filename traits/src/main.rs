trait Summary {
    fn summary(&self) -> String;
}

struct Article {
    title: String,
}

impl Summary for Article {
    fn summary(&self) -> String {
        format!("Article: {}", self.title)
    }
}

fn print_summary(item: &impl Summary) {
    println!("{}", item.summary());
}

fn main() {
    print_summary(&Article {
        title: "Learning Rust traits".to_string(),
    });
}
