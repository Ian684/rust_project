struct Stack<T> {
    data: Vec<T>,
}

impl<T: std::fmt::Debug> Stack<T> {
    fn new() -> Self {
        Stack {
            data: Vec::new(),
        }
    }
    fn push(&mut self, val: T){
        self.data.push(val);     
    }
    fn pop(&mut self) -> Option<T>{
        if self.data.is_empty(){
            return None;
        }
        self.data.pop()
    }
    fn peek(&self) -> Option<&T>{
        self.data.last()
    }
    fn print_all(&self){
        for i in &self.data{
            println!("{:?}" , i);
        }
    }
}







fn main() {
    let mut stack: Stack<i64> = Stack::<i64>::new();
    stack.push(1);
    stack.push(1);
    stack.push(1);
    stack.push(1);
    stack.push(1);
    stack.pop();
    stack.peek();
    stack.print_all();
}

fn exercise_2507482817_sum(values: &[i32]) -> i64 {
    values.iter().map(|value| *value as i64).sum()
}

fn exercise_2507482817_maximum(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn exercise_2507482817_minimum(values: &[i32]) -> Option<i32> {
    values.iter().copied().min()
}

fn exercise_2507482817_squares(values: &[i32]) -> Vec<i64> {
    values.iter().map(|value| (*value as i64) * (*value as i64)).collect()
}

fn exercise_2507482817_filtered(values: &[i32], threshold: i32) -> Vec<i32> {
    values.iter().copied().filter(|value| *value >= threshold).collect()
}

#[cfg(test)]
mod exercise_2507482817_tests {
    use super::*;

    #[test]
    fn exercise_2507482817_sum_test() {
        assert_eq!(exercise_2507482817_sum(&[1, 2, 3]), 6);
    }
}
const EXERCISE_2507482817_76: i32 = 93;
const EXERCISE_2507482817_77: i32 = 94;
const EXERCISE_2507482817_78: i32 = 95;
const EXERCISE_2507482817_79: i32 = 96;
