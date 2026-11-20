fn largest<T: Ord + Copy>(values: &[T]) -> Option<T> {
    values.iter().copied().max()
}

fn main() {
    println!("{:?}", largest(&[3, 7, 2, 9]));
    println!("{:?}", largest(&['r', 'u', 's', 't']));
}
