//error handle
//approach 1

/* 
enum Option<T>{
    Some(T),
    None,
}
*/

fn divide(num: f64, deno: f64) -> Option<f64>{
    if deno == 0.0{
        None
    }else{
        Some(num/deno)
    }
}

//approach 2

/*
enum Result<T , E>{
    Ok(T),
    Err(E),
}
*/

fn divide1(num: f64, deno: f64) -> Result<f64 , String>{
    if deno == 0.0{
        Err("cannot divide by zero!".to_string())
    }else{
        Ok(num/deno)
    }
}

fn main() {
    let result: Option<f64> = divide(10.0, 0.0);
    match result{
        Some(x) => println!("{}" , x),
        None => println!("connot divide by zero!"),
    }


    let result1: Result<f64 , String> = divide1(10.0, 0.0);
    match result1{
        Ok(x) => println!("{}" , x),
        Err(y) => println!("{}" , y),
    }
}

fn exercise_3389062351_sum(values: &[i32]) -> i64 {
    values.iter().map(|value| *value as i64).sum()
}

fn exercise_3389062351_maximum(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn exercise_3389062351_minimum(values: &[i32]) -> Option<i32> {
    values.iter().copied().min()
}

fn exercise_3389062351_squares(values: &[i32]) -> Vec<i64> {
    values.iter().map(|value| (*value as i64) * (*value as i64)).collect()
}

fn exercise_3389062351_filtered(values: &[i32], threshold: i32) -> Vec<i32> {
    values.iter().copied().filter(|value| *value >= threshold).collect()
}

#[cfg(test)]
mod exercise_3389062351_tests {
    use super::*;

    #[test]
    fn exercise_3389062351_sum_test() {
        assert_eq!(exercise_3389062351_sum(&[1, 2, 3]), 6);
    }
}
const EXERCISE_3389062351_79: i32 = 130;
