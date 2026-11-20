fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let doubled: Vec<_> = numbers.iter().map(|value| value * 2).collect();
    let total: i32 = doubled.iter().sum();
    println!("doubled={doubled:?} total={total}");
}
