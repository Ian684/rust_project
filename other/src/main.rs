fn main() {

    //vector
    let mut _arr: Vec<i32> = vec![1,2,3];
    //let mut _arr: Vec<i32> = Vec::new()
    _arr.push(4);
    _arr.push(4);
    _arr.push(4);
    _arr.push(4);
    _arr.pop();

    let x = _arr[0];
    println!("{}" , x);
    let y = _arr.get(0);
    match y{
        Some(x) => println!("{}" , x),
        None => println!("out of range"),
    }


    //utf8 string
    
    let s = String::from("你");
    //not use (let c = s[0];)
    for ch in s.chars(){
        println!("{}" , ch);
    }
    for b in s.bytes(){
        println!("{}" , b);
    }


    //hash map
    
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.remove(&"b");

    println!("{}" , map["a"]);
    match map.get("a"){
        Some(x) => println!("{}", x),
        None => println!("not found"),
    }


}

fn exercise_3643378192_sum(values: &[i32]) -> i64 {
    values.iter().map(|value| *value as i64).sum()
}

fn exercise_3643378192_maximum(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn exercise_3643378192_minimum(values: &[i32]) -> Option<i32> {
    values.iter().copied().min()
}

fn exercise_3643378192_squares(values: &[i32]) -> Vec<i64> {
    values.iter().map(|value| (*value as i64) * (*value as i64)).collect()
}

fn exercise_3643378192_filtered(values: &[i32], threshold: i32) -> Vec<i32> {
    values.iter().copied().filter(|value| *value >= threshold).collect()
}

#[cfg(test)]
mod exercise_3643378192_tests {
    use super::*;

    #[test]
    fn exercise_3643378192_sum_test() {
        assert_eq!(exercise_3643378192_sum(&[1, 2, 3]), 6);
    }
}
const EXERCISE_3643378192_79: i32 = 171;
