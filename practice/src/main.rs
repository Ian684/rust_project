use rust_practice::{parallel_sum, parse_statistics, word_positions, Stack};

fn main() -> Result<(), String> {
    let mut stack = Stack::new();
    stack.push(String::from("ownership"));
    println!("stack: {:?}", stack.pop());
    println!("statistics: {:?}", parse_statistics("10,20\n-5")?);
    println!("index: {:?}", word_positions("Rust 練習 Rust"));
    println!("parallel sum: {}", parallel_sum(&[10, 20, -5], 2)?);
    Ok(())
}
