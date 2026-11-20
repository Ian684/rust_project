#[derive(Clone, Debug)]
struct Item {
    id: usize,
    name: String,
    score: i32,
}

impl Item {
    fn new(id: usize, name: &str, score: i32) -> Self {
        Self { id, name: name.to_string(), score }
    }

    fn grade(&self) -> &'static str {
        if self.score >= 90 { "A" }
        else if self.score >= 80 { "B" }
        else if self.score >= 70 { "C" }
        else { "D" }
    }
}

fn summarize(items: &[Item]) -> (i32, i32, usize) {
    let sum: i32 = items.iter().map(|item| item.score).sum();
    let maximum = items.iter().map(|item| item.score).max().unwrap_or(0);
    (sum, maximum, items.len())
}

fn main() {
    println!("Rust exercise 19: strings");
    let mut items = vec![
        Item::new(1, "Ada", 92),
        Item::new(2, "Linus", 84),
        Item::new(3, "Grace", 78),
    ];

    items.sort_by_key(|item| item.id);
    for item in &items {
        println!("{} {} {}", item.id, item.name, item.grade());
    }

    let (sum, maximum, count) = summarize(&items);
    println!("Sum: {}", sum);
    println!("Maximum: {}", maximum);
    println!("Count: {}", count);

    let mut copied = items.clone();
    copied.reverse();
    println!("First after reverse: {:?}", copied.first());

    let text = items
        .iter()
        .map(|item| item.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    println!("Names: {}", text);
}
