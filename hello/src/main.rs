fn hello(x: i32 , y: u64 , z: f64) -> f64{
    println!("{} {} {}" , x , y , z);
    (x + y as i32) as f64 * z
}
fn main() {
    let x: i32 = -43;
    let y: u64 = 302;
    let z: f64 = 2.53;
    let a: bool = true;
    let b: String = "aaa".to_string();
    println!("signed: {}" , x);
    println!("unsigned: {}" , y);
    println!("float: {}" , z);
    println!("bool: {}" , a);
    println!("String: {}" , b);
    

    let arr: [i64;5] = [1,2,3,4,5];
    let brr: [&str;3] = ["a" , "b" , "c"];
    println!("{:?}" , arr);
    println!("{}" , brr[0]);
    println!("{}" , brr[1]);
    println!("{}" , brr[2]);


    let tuple1: (String , i64 , bool) = ("a".to_string() , 43 , true);
    println!("{:?}" , tuple1);
    let tuple2 = ("b" , 242 , false , [1,3,34,4]);
    println!("{:?}" , tuple2);


    let slice1: &[i64] = &[1,34,4,4,4];
    println!("{:?}" , slice1);
    let slice2: &[&str] = &["a" , "b" , "c"];
    println!("{:?}" , slice2);
    let slice3: &[&String] = &[&"a".to_string() , &"b".to_string() , &"c".to_string()];
    println!("{:?}" , slice3);
    //str &str String difference
    

    let c: f64 = hello(x , y , z);
    print!("{}\n" , c);


    
}

fn exercise_754077114_sum(values: &[i32]) -> i64 {
    values.iter().map(|value| *value as i64).sum()
}

fn exercise_754077114_maximum(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn exercise_754077114_minimum(values: &[i32]) -> Option<i32> {
    values.iter().copied().min()
}

fn exercise_754077114_squares(values: &[i32]) -> Vec<i64> {
    values.iter().map(|value| (*value as i64) * (*value as i64)).collect()
}

fn exercise_754077114_filtered(values: &[i32], threshold: i32) -> Vec<i32> {
    values.iter().copied().filter(|value| *value >= threshold).collect()
}

#[cfg(test)]
mod exercise_754077114_tests {
    use super::*;

    #[test]
    fn exercise_754077114_sum_test() {
        assert_eq!(exercise_754077114_sum(&[1, 2, 3]), 6);
    }
}
const EXERCISE_754077114_76: i32 = 90;
const EXERCISE_754077114_77: i32 = 91;
const EXERCISE_754077114_78: i32 = 92;
const EXERCISE_754077114_79: i32 = 93;
