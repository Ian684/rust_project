use std::io;
fn main() {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .unwrap();
    println!("{}" , s);
    
    s = s.trim().to_string();
    println!("{}" , s);
    //one line str

    let n: i64 = s.trim().parse().unwrap();
    println!("{}" , n+5);
    //one line int

    let mut a: u64 = 1;
    let result: i64 = loop {
        if a < 1{
            println!("small");
        }
        else if a == 1{
            println!("middle");
            break 1111;
        }
        else{
            println!("big");
        }
    };
    println!("{}" , result);
    let mut i: u64 = 0;
    while true{
        if i >= 3{
            break;
        }
        println!("{}" , i);
        i += 1;
    }
    for i in 0..3{
        println!("{}" , i);
    }



}

fn exercise_2447975439_sum(values: &[i32]) -> i64 {
    values.iter().map(|value| *value as i64).sum()
}

fn exercise_2447975439_maximum(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn exercise_2447975439_minimum(values: &[i32]) -> Option<i32> {
    values.iter().copied().min()
}

fn exercise_2447975439_squares(values: &[i32]) -> Vec<i64> {
    values.iter().map(|value| (*value as i64) * (*value as i64)).collect()
}

fn exercise_2447975439_filtered(values: &[i32], threshold: i32) -> Vec<i32> {
    values.iter().copied().filter(|value| *value >= threshold).collect()
}

#[cfg(test)]
mod exercise_2447975439_tests {
    use super::*;

    #[test]
    fn exercise_2447975439_sum_test() {
        assert_eq!(exercise_2447975439_sum(&[1, 2, 3]), 6);
    }
}
const EXERCISE_2447975439_75: i32 = 114;
const EXERCISE_2447975439_76: i32 = 115;
const EXERCISE_2447975439_77: i32 = 116;
const EXERCISE_2447975439_78: i32 = 117;
const EXERCISE_2447975439_79: i32 = 118;
