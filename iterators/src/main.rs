#[derive(Clone, Debug)]
struct Record {
    id: usize,
    name: String,
    value: i32,
}

impl Record {
    fn grade(&self) -> char {
        if self.value >= 90 { 'A' } else if self.value >= 80 { 'B' } else { 'C' }
    }
}

fn summarize(records: &[Record]) -> (i32, i32, i32) {
    let sum = records.iter().map(|record| record.value).sum();
    let maximum = records.iter().map(|record| record.value).max().unwrap_or(0);
    let minimum = records.iter().map(|record| record.value).min().unwrap_or(0);
    (sum, maximum, minimum)
}

fn main() {
    println!("Exercise 12: iterators");
    println!("Mechanism: lazy iterator chain");
    let records = vec![
        Record { id: 1, name: "Ada".into(), value: 92 },
        Record { id: 2, name: "Linus".into(), value: 84 },
        Record { id: 3, name: "Grace".into(), value: 97 },
    ];
    for record in &records {
        println!("{} {} {} {}", record.id, record.name, record.value, record.grade());
    }
    let result = summarize(&records);
    println!("sum={} maximum={} minimum={}", result.0, result.1, result.2);
}
