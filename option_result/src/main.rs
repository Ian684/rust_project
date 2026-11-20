#[derive(Clone, Debug)]
struct Record {
    id: usize,
    label: String,
    value: i32,
}

impl Record {
    fn new(id: usize, label: &str, value: i32) -> Self {
        Self { id, label: label.to_string(), value }
    }
}

fn total(records: &[Record]) -> i32 {
    records.iter().map(|record| record.value).sum()
}

fn maximum(records: &[Record]) -> Option<i32> {
    records.iter().map(|record| record.value).max()
}

fn main() {
    println!("Rust exercise 27: option_result");
    let mut records = vec![
        Record::new(1, "Ada", 92),
        Record::new(2, "Linus", 84),
        Record::new(3, "Grace", 97),
        Record::new(4, "Ken", 78),
    ];

    records.sort_by_key(|record| record.id);
    for record in &records {
        println!("{} {} {}", record.id, record.label, record.value);
    }

    println!("Total: {}", total(&records));
    println!("Maximum: {:?}", maximum(&records));

    let names = records
        .iter()
        .map(|record| record.label.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    println!("Names: {}", names);

    let mut reversed = records.clone();
    reversed.reverse();
    println!("Reversed: {:?}", reversed);
}
